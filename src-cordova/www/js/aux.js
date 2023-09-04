var dingSound;
var bowlSound;

export function setMedia() {
  dingSound = new Media('/android_asset/www/assets/sounds/ding.ogg');
  bowlSound = new Media('/android_asset/www/assets/sounds/bowl.ogg');
}

export function playDing(volume) {
  dingSound.setVolume(volume);
  dingSound.play();
}

export function playBowl(volume) {
  bowlSound.setVolume(volume);
  bowlSound.play();
}

export function startForegroundService() {
  cordova.plugins.backgroundMode.on('activate', function() {
    cordova.plugins.backgroundMode.disableWebViewOptimizations(); 
  });
  cordova.plugins.backgroundMode.enable();

  cordova.plugins.foregroundService.start('Meditation session', 'Running in background', 'ac_bell');
}

export function stopForegroundService() {
  cordova.plugins.backgroundMode.disable();

  cordova.plugins.foregroundService.stop();
}
