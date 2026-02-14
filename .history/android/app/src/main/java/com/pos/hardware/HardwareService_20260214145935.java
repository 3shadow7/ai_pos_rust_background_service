package com.pos.hardware;

import android.app.Notification;
import android.app.NotificationChannel;
import android.app.NotificationManager;
import android.app.Service;
import android.content.Intent;
import android.os.Build;
import android.os.IBinder;
import androidx.core.app.NotificationCompat;

public class HardwareService extends Service {
    // Load the Rust library
    static {
        System.loadLibrary("pos_hardware_lib");
    }

    // Declare the native method from Rust
    public native void startServer(int port);

    @Override
    public void onCreate() {
        super.onCreate();
        createNotificationChannel();
    }

    @Override
    public int onStartCommand(Intent intent, int flags, int startId) {
        // Start Foreground Service to keep it alive
        Notification notification = new NotificationCompat.Builder(this, "POSTChannel")
                .setContentTitle("POS Hardware Service")
                .setContentText("Listening for printer commands...")
                .setSmallIcon(android.R.drawable.ic_menu_rotate)
                .build();

        startForeground(1, notification);

        // Start Rust Server
        startServer(8080); // Default port

        return START_STICKY;
    }

    @Override
    public IBinder onBind(Intent intent) {
        return null;
    }

    private void createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            NotificationChannel serviceChannel = new NotificationChannel(
                    "POSTChannel",
                    "POS Hardware Service Channel",
                    NotificationManager.IMPORTANCE_DEFAULT
            );
            NotificationManager manager = getSystemService(NotificationManager.class);
            manager.createNotificationChannel(serviceChannel);
        }
    }
}
