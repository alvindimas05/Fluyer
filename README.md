<p align="center">
  <img src="assets/icon.png" width="128" alt="Fluyer Icon" />
</p>

<h1 align="center">🦋🎵 Fluyer</h1>
<p align="center">A Beautiful Cross-Platform Music Player.</p>

<p align="center">
  <a href="https://github.com/alvindimas05/Fluyer/stargazers">
    <img alt="GitHub stars" src="https://img.shields.io/github/stars/alvindimas05/Fluyer?style=flat-square&color=ffd700">
  </a>
  <a href="https://github.com/alvindimas05/Fluyer/releases/latest">
    <img alt="GitHub release" src="https://img.shields.io/github/v/release/alvindimas05/Fluyer?style=flat-square&color=blueviolet">
  </a>
  <a href="https://github.com/alvindimas05/Fluyer/releases">
    <img alt="Total Downloads" src="https://img.shields.io/github/downloads/alvindimas05/Fluyer/total?style=flat-square&color=brightgreen">
  </a>
</p>

> [!NOTE]
> This project is a slow-paced hobby. I’m building it just for fun :)

---

<p align="center">
  <img src="assets/preview1.png" alt="Fluyer Preview 1" width="45%"/>
  <img src="assets/preview2.png" alt="Fluyer Preview 2" width="45%"/>
</p>

---

## 📦 Downloads

> [!WARNING]
> 🚧 This app is a beta version and still under development. It may contain bugs or missing features. Check the issues below before installing.

You can download from the latest release from the [Releases Page](https://github.com/alvindimas05/Fluyer/releases/latest).

---

## 🐞 Known Issues

### macOS

> [!IMPORTANT]
> 🔐 The app is **not notarized** due to Apple Developer fees. macOS will warn that Fluyer “cannot be verified.” This is expected and harmless.

#### macOS 14 (Sonoma) and below:
1. Right-click `Fluyer.app`
2. Select **Open**
3. Confirm again in the dialog

#### macOS 15 (Sequoia) and above:
1. Open the app — it will be blocked
2. Go to `System Settings > Privacy & Security`
3. Scroll down and click **Open Anyway**
4. Confirm and authenticate
5. Open again from **Applications**

---

### Linux

> [!IMPORTANT]
> 🧩 If you're using the `.deb`, you **must install libmpv** manually.

Example for Ubuntu:
```bash
sudo curl --output-dir /etc/apt/trusted.gpg.d -O https://apt.fruit.je/fruit.gpg
echo "deb http://apt.fruit.je/debian trixie mpv" | sudo tee /etc/apt/sources.list.d/fruit.list
sudo apt-get update -y
sudo apt-get install -y libmpv2
```

---

### Universal

- 🚫 **High resource usage** due to animated background. It’s currently disabled, but the background image remains enabled.

---

### iOS

> [!IMPORTANT]
> 🍏 **iOS version is not available.** Although previously tested, I don’t plan to release it due to Apple Developer fees.

---

## ❤️ Contributing

PRs and feedback are welcome, but again—this project is made for fun!

---

## 📄 License

GNU General Public License v3.0
