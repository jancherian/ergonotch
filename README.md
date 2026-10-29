# Notch Companion

> **An ambient desktop companion for focus, ergonomics, and daily wellbeing.**  
> Lives quietly at the top of your screen as a translucent dark island. 100% offline, zero telemetry, local-first.

[![Live Web Preview](https://img.shields.io/badge/Live%20Demo-GitHub%20Pages-success?style=for-the-badge&logo=github)](https://jancherian.github.io/ergonotch/)
[![Windows Installer](https://img.shields.io/badge/Download-Windows%20Setup%20.exe-blueviolet?style=for-the-badge&logo=windows)](https://github.com/jancherian/ergonotch/releases)

---

## 🌐 Quick Access (Evaluators & Judges)

- **🚀 Public Live Web Demo (One-Click, No Install):**  
  👉 **[https://jancherian.github.io/ergonotch/](https://jancherian.github.io/ergonotch/)**  
  *Runs the complete interactive companion directly in your browser. Camera posture analysis and eye fatigue detection run in-browser using WebAssembly and local models from the same origin (no CDN, zero external calls).*
- **📦 Windows Desktop Installer (.exe):**  
  👉 **[Download Notch-Setup.exe (v1.0.0)](https://github.com/jancherian/ergonotch/releases/download/v1.0.0/Notch-Setup.exe)** (25.8 MB, NSIS `currentUser` installer, no admin required).  
  *Release Page: [GitHub Releases v1.0.0](https://github.com/jancherian/ergonotch/releases/tag/v1.0.0)*
- **🛡️ Windows SmartScreen Note:**  
  *The installer is unsigned (standard for hackathon projects). If Windows SmartScreen prompts on first launch, click **"More info"** and then **"Run anyway"**.*

---

## 🌟 Overview

**Notch Companion** is a Windows desktop application that bridges deep work productivity and physical ergonomics. Rather than overwhelming you with notifications or full-screen windows, Notch lives as a discreet, floating dynamic island at the top of your display, expanding on demand and staying in your peripheral vision while working.

All vision and machine-learning models run **100% locally on your computer** using WebAssembly and bundled `.task` models. **Zero bytes of video or personal telemetry ever leave your device.**

---

## ✨ Key Features

### 1. ⏱️ Focus Engine
- Structured work and break sessions (25m / 5m default, configurable).
- Active task binding: assign your primary task directly to the timer.
- Real-time collapsed pill tracking (`⏱️ 24:58 · [Active Task]`).
- Interruption tracking and session completion logging without drift.
- Mochi companion reacts to work vs break states.

### 2. 📋 Tasks Manager
- Full task management with priorities (`Low`, `Medium`, `High`), due dates, and times.
- Clear filtering: `Today`, `Upcoming`, `Overdue`, `All`, and `Done`.
- Distinct overdue badges that keep missed deadlines visible without anxiety.
- One-tap "Set Active 🎯" to push tasks straight to the Focus timer.

### 3. 🔥 Daily Habits & Streaks
- Track daily and weekday micro-habits (stretch, hydrate, read, walk).
- Visual 7-day completion tracker (`M T W T F S S`).
- **Authentic streaks:** Missed days are **never auto-completed** and stay visible as missed.
- One-tap completion with audio feedback.
- Pause, resume, and archive habits at any time.

### 4. 🧍 100% Offline Posture Analysis
- Real-time pose landmarking powered by local `@mediapipe/tasks-vision` (running completely offline via bundled WASM and `pose_landmarker_lite.task`).
- Detects slouch ratio, forward head posture, and shoulder tilt angle relative to your calibrated baseline.
- **Privacy-first:** Video stream is processed in-memory and frames are discarded immediately. Camera turns off instantly when paused or stopped.
- Persistent hold threshold (20s hold required before warning; 5s in Demo Mode).
- One-tap baseline calibration: *"Sit straight, then press Calibrate"*.

### 5. 👁️ Facial Fatigue & PERCLOS Monitoring
- Real-time Face Landmarker with eye-blendshapes (`eyeBlinkLeft`/`eyeBlinkRight`).
- Rolling 60-second PERCLOS (% time eye closed) and prolonged blink closure tracking (>1.5s).
- Non-diagnostic soft suggestions (*"☕ You might be getting tired. Consider a short break"*).
- Independent toggle: can run posture tracking with or without facial analysis.

### 6. 🧘 Guided Wellbeing Micro-Breaks
- 4 targeted guided micro-breaks:
  - 👁️ **20-20-20 Eye Rest** (20s)
  - 🧘 **Desk Stretch** (60s)
  - 🧍 **Spine & Posture Reset** (30s)
  - 💧 **Hydration Check** (30s)
- Focus deferral: suppresses non-critical reminders during active focus sessions until break time.
- Snooze (5m), dismiss, and completion logging.

### 7. 📊 Insights & Daily Summary
- Measured activity tiles vs estimates: Focus time, Tasks completed, Habits consistency, Breaks taken.
- Interactive 7-day focus bar chart with hover tooltips and live today indicator.
- Screen & App activity breakdown (sample data reference).
- Prominently anchored philosophy: *"Descriptive insights, not a measure of productivity."*

### 8. ⚙️ Settings, Privacy & Presentation Demo Mode
- Toggles: Launch at sign-in, camera analysis, facial analysis, usage tracking, UI animations, notification frequency, hover sensitivity, collapse delay.
- **Presentation Demo Mode:** Shortens alert thresholds to 5 seconds for rapid live demonstration.
- **Hidden Demo Reset (<kbd>Ctrl+Shift+R</kbd>):** Instantly restores a clean, fresh demonstration state with realistic seed data.
- **Delete All My Data:** Complete local storage purge with safety confirmation.

---

## 🔒 Privacy & Security Guarantee

- **Zero Network Traffic:** Zero external telemetry sockets, analytics, or remote API calls. Works completely offline without an internet connection.
- **Local WASM Inference:** MediaPipe vision inference runs on your CPU/GPU via WebAssembly in the WebView sandbox.
- **Zero Video Saved:** Camera frames are evaluated frame-by-frame in volatile memory and immediately freed. No screenshots, images, or video clips are ever saved to disk or transmitted.
- **Zero Keystrokes or Screen Capture:** Notch Companion never monitors, reads, or records your keystrokes, clipboard, or screen contents.

---

## 🚀 Running & Building

### Prerequisites
- Node.js (v18+)
- Rust & Cargo (1.75+)
- WebView2 Runtime (pre-installed on Windows 10/11)

### Development
```powershell
cd coucou-windows-v0.5.3/windows
npm install
npm run dev
```

### Build Production Installer
```powershell
cd coucou-windows-v0.5.3/windows
npm run build
npx tauri build
```
The installer is generated in `src-tauri/target/release/bundle/nsis/Notch_0.1.0_x64-setup.exe`.

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
| :--- | :--- |
| <kbd>Ctrl+Shift+R</kbd> | **Hidden Demo Reset** (Restores clean presentation state) |
| <kbd>Ctrl+P</kbd> | **Pin / Unpin** Island |
| <kbd>1</kbd> | Jump to **Focus** view |
| <kbd>2</kbd> | Jump to **Tasks** view |
| <kbd>3</kbd> | Jump to **Habits** view |
| <kbd>4</kbd> | Jump to **Wellbeing** view |
| <kbd>5</kbd> | Jump to **Insights** view |
| <kbd>6</kbd> | Jump to **Settings** view |
| <kbd>Space</kbd> | Start / Pause Focus Timer (when on Focus tab) |
| <kbd>Esc</kbd> | Collapse Island |

---

## 📚 Credits & Licenses

- **Coucou Core:** Based on the Coucou project by Louis Raille (MIT License).
- **Google MediaPipe:** Vision models and WebAssembly runtime (Apache 2.0).
- **Mochi Mascot:** Hand-crafted animations and SVG art from Coucou (MIT).
- **Tauri Framework:** Safe, lightweight native desktop runtime (MIT / Apache 2.0).
