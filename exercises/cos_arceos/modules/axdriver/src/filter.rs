use cfg_if::cfg_if;
use driver_common::{BaseDriverOps, DevResult, DeviceType};

cfg_if! {
    if #[cfg(bus = "pci")] {
        use driver_pci::{PciRoot, DeviceFunction, DeviceFunctionInfo};
        type VirtIoTransport = driver_virtio::PciTransport;
    } else if #[cfg(bus =  "mmio")] {
        type VirtIoTransport = driver_virtio::MmioTransport;
    }
}

pub struct NetFilter<T> {
    pub inner: T,
}

impl<T: BaseDriverOps> BaseDriverOps for NetFilter<T> {
    fn device_name(&self) -> &str {
        self.inner.device_name()
    }

    fn device_type(&self) -> DeviceType {
        self.inner.device_type()
    }
}

cfg_if! {
if #[cfg(net_dev = "virtio-net")] {
    use driver_net::{NetBufPtr, NetDriverOps};
    use driver_virtio::{Transport, VirtIoHal, VirtIoNetDev};

    impl<H, T, const QS: usize> NetDriverOps for NetFilter<VirtIoNetDev<H, T, QS>>
    where
        H: VirtIoHal,
        T: Transport,
    {
        fn mac_address(&self) -> driver_net::EthernetAddress {
            self.inner.mac_address()
        }

        fn can_transmit(&self) -> bool {
            self.inner.can_transmit()
        }

        fn can_receive(&self) -> bool {
            self.inner.can_receive()
        }

        fn rx_queue_size(&self) -> usize {
            self.inner.rx_queue_size()
        }

        fn tx_queue_size(&self) -> usize {
            self.inner.tx_queue_size()
        }

        fn recycle_rx_buffer(&mut self, rx_buf: NetBufPtr) -> DevResult {
            self.inner.recycle_rx_buffer(rx_buf)
        }

        fn recycle_tx_buffers(&mut self) -> DevResult {
            self.inner.recycle_tx_buffers()
        }

        fn transmit(&mut self, tx_buf: NetBufPtr) -> DevResult {
            log::warn!("Filter: transmit len [{}]", tx_buf.packet_len());
            self.inner.transmit(tx_buf)
        }

        fn receive(&mut self) -> DevResult<NetBufPtr> {
            let ret = self.inner.receive()?;
            log::warn!("Filter: receive len[{:?}]", ret.packet_len());
            Ok(ret)
        }

        fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<NetBufPtr> {
            self.inner.alloc_tx_buffer(size)
        }
    }
}}
