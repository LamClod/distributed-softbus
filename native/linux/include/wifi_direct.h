#ifndef SOFTBUS_WIFI_DIRECT_H
#define SOFTBUS_WIFI_DIRECT_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief 初始化Wi-Fi Direct
 * @return 0表示成功，-1表示失败
 */
int wifi_direct_init(void);

/**
 * @brief 清理Wi-Fi Direct
 * @return 0表示成功，-1表示失败
 */
int wifi_direct_cleanup(void);

/**
 * @brief 开始广播
 * @param device_name 设备名称
 * @return 0表示成功，-1表示失败
 */
int wifi_direct_start_advertising(const char* device_name);

/**
 * @brief 停止广播
 * @return 0表示成功，-1表示失败
 */
int wifi_direct_stop_advertising(void);

#ifdef __cplusplus
}
#endif

#endif // SOFTBUS_WIFI_DIRECT_H
