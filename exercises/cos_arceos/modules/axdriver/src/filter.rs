use driver_common::{BaseDriverOps, DevResult, DeviceType};
use driver_net::NetDriverOps;
use driver_virtio::{Transport, VirtIoHal, VirtIoNetDev};

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

    fn recycle_rx_buffer(&mut self, rx_buf: driver_net::NetBufPtr) -> DevResult {
        self.inner.recycle_rx_buffer(rx_buf)
    }

    fn recycle_tx_buffers(&mut self) -> DevResult {
        self.inner.recycle_tx_buffers()
    }

    fn transmit(&mut self, tx_buf: driver_net::NetBufPtr) -> DevResult {
        log::warn!("Filter: transmit len [{}]", tx_buf.packet_len());
        self.inner.transmit(tx_buf)
    }

    fn receive(&mut self) -> DevResult<driver_net::NetBufPtr> {
        let ret = self.inner.receive()?;
        log::warn!("Filter: receive len[{:?}]", ret.packet_len());
        Ok(ret)
    }

    fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<driver_net::NetBufPtr> {
        self.inner.alloc_tx_buffer(size)
    }
}
