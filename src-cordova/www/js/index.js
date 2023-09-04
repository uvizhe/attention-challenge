import * as aux from "./aux.js";

document.addEventListener('deviceready', onDeviceReady, false);

function onDeviceReady() {
  // NOTE: `cordova-plugin-background-mode` is outdated but can still be useful
  // in combination with `cordova-plugin-foreground-service`.
  // This is required(?) for background service to work with Android 12+
  cordova.plugins.backgroundMode.setDefaults({ silent: true });

  aux.setMedia();

  (async () => {
      const { default: init, main } = await import("./attention_challenge.js");
      init().then(() => {
          main();
      });
  })();
}
