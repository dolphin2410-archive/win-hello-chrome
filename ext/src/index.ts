export {}
declare global {
  var Biometrics: typeof _Biometrics
}

class _Biometrics {
  static call(callback: (r: any) => void = (response) => console.log("Received: " + JSON.stringify(response))
  ) {
    chrome.runtime.sendNativeMessage('me.dolphin2410.chrome_native',
    { task: "HelloTask", message: "Nununana"}, callback)
  }
}

window.globalThis.Biometrics = _Biometrics

let communication = document.getElementById("communication");

communication?.addEventListener("click", async () => {
  Biometrics.call()
});