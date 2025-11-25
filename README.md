# Deadline Tracker

>[!WARNING]
> The project is not finished. And used lots of vibe coding.

Based on Rust and Dioxus.

You could track your deadlines through both List / Calender views.

TODO List:

- [ ] Milestone UI integration
- [ ] multi-language support
- [ ] Settings panel and storage
- [ ] Support Narrow View and android
- [ ] DIY Urgency function


version: v0.0.1

> [!Note] macOS builds

> Unsigned development builds downloaded from GitHub triggers Gatekeeper with a *"DeadlineTracker is damaged"* dialog. The binaries are fine—they just are not code signed or notarized yet. After downloading, clear the quarantine attribute before launching:

> ```bash
> xattr -cr /Applications/Deadline\ Tracker.app
> ```

> Adjust the path if you keep the app elsewhere. Once we have an Apple Developer certificate the workflow can be updated to sign/notarize releases, but for now this manual step is required.