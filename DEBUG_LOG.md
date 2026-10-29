# Notch Companion — Debug & Polish Log

Running log of all bugs, triage, root cause analyses, fixes, and verification proofs.

---

## PART 3: Web Demo Fixes & Verification

### Bug 3.1: MediaPipe WASM and model asset 404s
- **Evidence:** Console error `Failed to fetch wasm binary at /wasm/vision_wasm_internal.wasm (404 Not Found)` when camera started on GitHub Pages subpath `/ergonotch/`.
- **Root Cause:** Hardcoded root-relative paths `"/wasm/"` resolved to `https://jancherian.github.io/wasm/...` instead of `https://jancherian.github.io/ergonotch/wasm/...`.
- **Fix:** Switched to document base URL resolution:
  `new URL("wasm/", document.baseURI).href` and `new URL("models/pose_landmarker_lite.task", document.baseURI).href`.
- **Proof:** E2E Playwright test `scripts/verify_e2e_web.mjs` confirmed zero 404s on network log.

### Bug 3.2: Sound audio file 404s on subpath
- **Evidence:** Audio fetch 404s for `sounds/*.wav` when hosted under subdirectories.
- **Root Cause:** Path `sounds/${name}.wav` was not resolved relative to `document.baseURI`.
- **Fix:** Used `new URL('sounds/' + name + '.wav', document.baseURI).href` in `sound.ts`.
- **Proof:** All sounds load cleanly with zero network errors.

### Bug 3.3: 4.9-second launch greeting blocked web preview
- **Evidence:** Visiting the web demo showed a blank black canvas or stuck greeting for nearly 5 seconds before the notch became interactive.
- **Root Cause:** Desktop launch greeting sequence `fsm.launch()` ran full 4.9s intro sequence before revealing the interactive notch.
- **Fix:** In web preview mode, bypass the intro delay and invoke `island.fsm.reveal()` directly.
- **Proof:** Instant interactive pill appears on page load (< 200ms).

### Bug 3.4: Toggle Notch button & Pill click did not open default view
- **Evidence:** Clicking "Toggle Notch (Simulate Hover/Click)" or clicking the pill did not expand the panel.
- **Root Cause:** FSM click state required explicit mode change and `setPinned(true)`.
- **Fix:** Added fallback click handlers to trigger `island.expand(State.defaultView())` and `island.setPinned(true)`.
- **Proof:** E2E Playwright test verified click expands to 720px panel and all 6 tabs interact smoothly.

---

## PART 4: Windows Desktop Installer & Single-Instance Lock

### Bug 4.1: "Nothing happened" when clicking installed executable
- **Evidence:** Executable exited within 1 second with ExitCode 101. No window appeared.
- **Root Cause:**
  1. A previous zombie process was holding the WebView2 User Data Folder (`EBWebView`) lock.
  2. `tauri-plugin-single-instance` used Win32 `FindWindowW` which failed for frameless tool windows, falling through to initialize WebView2 twice and crashing with `HRESULT(0x800700AA): The requested resource is in use`.
  3. C runtime loader left non-zero error in `GetLastError()`, causing naive `CreateMutexW` checks to trigger false-positive secondary instance exits.
- **Fix:**
  1. Added explicit `SetLastError(WIN32_ERROR(0))` before `CreateMutexW` in `main.rs`.
  2. Guarded single-instance check in native Win32 before Tauri or WebView2 initialization; secondary instances exit immediately with code 0 without touching WebView2 lock.
  3. Removed conflicting `tauri_plugin_single_instance` from `lib.rs`.
  4. Added panic hook and file logger writing to `%LOCALAPPDATA%\Notch\logs\app.log`.
  5. Renamed binary to `notch.exe` and product name to `Notch`.
- **Proof:**
  - `Notch-Setup.exe /S` installed cleanly to `%LOCALAPPDATA%\Notch\notch.exe`.
  - Process runs cleanly with PID in Task Manager, working set ~54MB, zero TCP connections.
  - Secondary instance launched via shortcut exits with code 0, primary instance remains running uninterrupted.

---

## PART 5: Animation Smoothness & 60 FPS Target

### Bug 5.1: Hover flicker from fast cursor sweeps
- **Evidence:** Moving cursor quickly past top of screen accidentally expanded the entire notch panel.
- **Root Cause:** `fsm.mouseEntered()` immediately transitioned from `hidden`/`petit` to `home` without hysteresis debounce.
- **Fix:** Added 130ms hover enter hysteresis in `fsm.ts` (`hoverEnterTimer`). Fast sweeps (<100ms) cancel the timer before expansion occurs; intentional hovers (>130ms) open smoothly.
- **Proof:** `scripts/verify_animations.mjs` Test 1 confirmed fast 60ms cursor sweep leaves pill collapsed at 288px (`Resisted open: true`), while intentional hover expands to 644px.

### Bug 5.2: Slow, sluggish spring response curve
- **Evidence:** Expand animation took > 500ms and felt loose/heavy.
- **Root Cause:** Default spring configuration in `anim.ts` was `response = 0.5` (500ms) with `damping = 0.72`, and collapse ease was 340ms.
- **Fix:** Tuned `Spring` and `Tracked` in `anim.ts` to `response = 0.32, damping = 0.78` (~300ms snappy expand) and collapse duration to 280ms with cubic-bezier ease curve.
- **Proof:** Measured 60.3 FPS with average frame delta of 16.58ms across multiple expand/collapse cycles.

### Bug 5.3: Main thread jank during posture inference
- **Evidence:** MediaPipe pose landmarker running on main thread could drop frames during expand/collapse transitions.
- **Root Cause:** Inference loop ran continuously every 200ms regardless of window animation state.
- **Fix:** Added `State.isGeometryAnimating` flag. In `posture.ts`, inference skips frames while the notch is expanding or collapsing (280-320ms) so CPU/GPU is 100% dedicated to 60fps rendering. Throttled collapsed posture checks to ~2 fps (500ms) to conserve CPU/battery.
- **Proof:** Playwright benchmark reported **0 long frames (>25ms)** and **0 jank frames (>50ms)** during 5 continuous expand/collapse cycles.

### Bug 5.4: Animation respect for `prefers-reduced-motion` and settings
- **Evidence:** Users with accessibility reduced motion or disabled animations still experienced physics spring movement.
- **Root Cause:** `animateGeometry` always executed `springTo()` and `curveTowards()`.
- **Fix:** Added `shouldAnimate()` checking `window.matchMedia("(prefers-reduced-motion: reduce)").matches` and `body.no-animations`. Snaps geometry immediately via `this.width.jump(w)` with zero duration, and added CSS reduced-motion overrides.
- **Proof:** `verify_animations.mjs` Test 4 confirmed immediate width snap under reduced motion in < 30ms.

---

## PART 8: Documentation, Verification & Cleanup

### Bug 8.1: Hardcoded absolute `file:///` URLs in documentation
- **Evidence:** `README.md` and `SUMMARY.md` contained machine-specific absolute file links (`file:///c:/Users/Jan%20Cherian%20Thomas/...`).
- **Root Cause:** Artifact paths were exported as local OS file URLs instead of relative repo paths.
- **Fix:** Converted all file links to relative Markdown links (`./DEMO_SCRIPT.md`, `.github/workflows/release.yml`, `./README.md`).
- **Proof:** Workspace search across all `.md` files confirmed zero remaining `file:///c:/Users` links.

### Bug 8.2: Outdated local PID and executable naming in summary
- **Evidence:** `SUMMARY.md` contained `coucou.exe (PID 5092)`.
- **Root Cause:** Legacy development process label was not updated after renaming to `notch.exe` in Part 4.
- **Fix:** Replaced with `notch.exe (Windows 10/11 x64)` and verified single-instance execution.
- **Proof:** Process table and log inspection confirms `notch.exe` running without any Coucou references.

### Bug 8.3: Verification of "0 outbound sockets" claim
- **Evidence:** Privacy claim required verifiable command-line proof on the production executable.
- **Root Cause:** Unsubstantiated claims violate project rules ("Never claim something is fixed without proof").
- **Fix:** Documented exact reproducible command in README & Summary:
  `Get-NetTCPConnection -OwningProcess (Get-Process notch).Id -ErrorAction SilentlyContinue` which returns 0 open TCP connections.
- **Proof:** Re-ran against running `notch.exe` instance; confirmed 0 outbound sockets.

### Bug 8.4: Missing Troubleshooting guidance for judges
- **Evidence:** Users encountering SmartScreen or running a secondary instance without closing the first could see nothing happen.
- **Root Cause:** Documentation lacked clear instructions on Windows SmartScreen, WebView2 Runtime requirements, and single-instance mutex mechanics.
- **Fix:** Added dedicated **Troubleshooting** table to `README.md` covering SmartScreen bypass, single-instance mutex checking via system tray, and `%LOCALAPPDATA%\Notch\logs\app.log` inspection.
- **Proof:** Reviewed in `README.md`.

---
