using System;
using System.Runtime.InteropServices;
using Windows.Devices.Bluetooth;
using Windows.Devices.Bluetooth.Advertisement;

namespace SoftBus.Native.Windows
{
    /// <summary>
    /// Windows BLE适配器实现
    /// </summary>
    public class BleAdapter
    {
        private BluetoothLEAdvertisementWatcher? _watcher;

        public BleAdapter()
        {
        }

        /// <summary>
        /// 初始化BLE适配器
        /// </summary>
        public void Initialize()
        {
            Console.WriteLine("Initializing Windows BLE adapter");
            _watcher = new BluetoothLEAdvertisementWatcher();
            _watcher.Received += OnAdvertisementReceived;
        }

        /// <summary>
        /// 开始扫描
        /// </summary>
        public void StartScan()
        {
            if (_watcher == null)
                throw new InvalidOperationException("Adapter not initialized");

            _watcher.Start();
            Console.WriteLine("BLE scan started");
        }

        /// <summary>
        /// 停止扫描
        /// </summary>
        public void StopScan()
        {
            _watcher?.Stop();
            Console.WriteLine("BLE scan stopped");
        }

        private void OnAdvertisementReceived(
            BluetoothLEAdvertisementWatcher sender,
            BluetoothLEAdvertisementReceivedEventArgs args)
        {
            Console.WriteLine($"BLE device found: {args.BluetoothAddress:X}");
        }

        // FFI导出函数
        [UnmanagedCallersOnly(EntryPoint = "ble_adapter_init")]
        public static int Init()
        {
            try
            {
                var adapter = new BleAdapter();
                adapter.Initialize();
                return 0;
            }
            catch
            {
                return -1;
            }
        }
    }
}
