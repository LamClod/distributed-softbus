package com.softbus;

/**
 * Android原生适配器JNI接口
 */
public class NativeAdapter {
    
    static {
        System.loadLibrary("softbus_native");
    }

    /**
     * 初始化BLE适配器
     * @return 0表示成功，-1表示失败
     */
    public native int initBleAdapter();

    /**
     * 初始化Wi-Fi Direct适配器
     * @return 0表示成功，-1表示失败
     */
    public native int initWiFiDirectAdapter();

    /**
     * 开始BLE扫描
     * @return 0表示成功，-1表示失败
     */
    public native int startBleScan();

    /**
     * 停止BLE扫描
     * @return 0表示成功，-1表示失败
     */
    public native int stopBleScan();
}
