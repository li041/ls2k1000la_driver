# ls2k1000la_driver

仓库内提供c和rust语言实现的ahci和gmac设备驱动，适配龙芯2k1000la板卡

驱动测试操作系统为RT-Thread，c直接和OS一起编译，rust需要编译为库后再链接，通过cbingen和ffi实现调度

由于时间有限，驱动代码还不够完善，欢迎提交pr反馈🚀
