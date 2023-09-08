var dingSound;
var bowlSound;

export function setMedia() {
  dingSound = new Media('/android_asset/www/assets/sounds/ding.ogg');
  bowlSound = new Media('/android_asset/www/assets/sounds/bowl.ogg');
}

export function playDing(volume) {
  dingSound.play();
  dingSound.setVolume(volume);
}

export function playBowl(volume) {
  bowlSound.play();
  bowlSound.setVolume(volume);
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

export function enableDNDMode() {
  AudioManagement.setAudioMode(AudioManagement.AudioMode.SILENT, null, null);
}

export function disableDNDMode() {
  AudioManagement.setAudioMode(AudioManagement.AudioMode.NORMAL, null, null);
}

export function hasDNDPermission() {
  cordova.plugins.notification.local.hasDoNotDisturbPermissions((granted) => {
    if (granted) {
      document.dispatchEvent(new Event('has_dnd_granted'));
    } else {
      document.dispatchEvent(new Event('has_dnd_not_granted'));
    }
  });
}

export function requestDNDPermission() {
  cordova.plugins.notification.local.requestDoNotDisturbPermissions((granted) => {
    if (granted) {
      document.dispatchEvent(new Event('dnd_granted'));
    } else {
      document.dispatchEvent(new Event('dnd_not_granted'));
    }
  });
}
