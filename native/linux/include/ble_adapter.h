#ifndef SOFTBUS_BLE_ADAPTER_H
#define SOFTBUS_BLE_ADAPTER_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief 初始化BLE适配器
 * @return 0表示成功，-1表示失败
 */
int ble_adapter_init(void);

/**
 * @brief 清理BLE适配器
 * @return 0表示成功，-1表示失败
 */
int ble_adapter_cleanup(void);

/**
 * @brief 开始扫描
 * @return 0表示成功，-1表示失败
 */
int ble_adapter_start_scan(void);

/**
 * @brief 停止扫描
 * @return 0表示成功，-1表示失败
 */
int ble_adapter_stop_scan(void);

#ifdef __cplusplus
}
#endif

#endif // SOFTBUS_BLE_ADAPTER_H
