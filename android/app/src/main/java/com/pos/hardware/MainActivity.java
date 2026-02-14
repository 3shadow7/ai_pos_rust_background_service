package com.pos.hardware;

import android.app.Activity;
import android.content.Intent;
import android.os.Bundle;

public class MainActivity extends Activity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        
        // Auto-start the service
        Intent serviceIntent = new Intent(this, HardwareService.class);
        startForegroundService(serviceIntent);
        
        // Close activity immediately (it's just a launcher)
        finish();
    }
}
