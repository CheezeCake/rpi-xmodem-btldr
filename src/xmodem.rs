const SOH: u8 = 0x1;
const EOT: u8 = 0x4;
const ACK: u8 = 0x6;
const NAK: u8 = 0x15;

pub trait Recv {
    fn recv(&self, timeout: u32) -> Result<u8, Error>;

    fn recv_timeout(&self) -> Result<u8, Error> {
        const TIMEOUT_US: u32 = 10_000_000;
        self.recv(TIMEOUT_US)
    }
}

pub trait Send {
    fn send(&self, c: u8) -> Result<(), Error>;
}

#[derive(Debug)]
pub enum Error {
    MaxRetries(usize),
    InvalidHeader,
    ChecksumError,
    Timeout,
}

enum Header {
    BlockNumber(u8),
    EndOfTransmission,
}

pub struct XmodemReceiver<T>(T);

impl<T: Recv + Send> XmodemReceiver<T> {
    pub fn new(receiver: T) -> Self {
        Self(receiver)
    }

    pub fn start(&self) {
        self.0.send(NAK).unwrap();
    }

    pub fn recv_packet(&self, buf: &mut [u8]) -> Result<usize, Error> {
        const MAX_RETRIES: usize = 10;

        for _ in 0..MAX_RETRIES {
            match self.try_recv_packet(buf) {
                Ok(n) => {
                    self.0.send(ACK)?;
                    return Ok(n);
                }
                Err(_) => {
                    self.0.send(NAK)?;
                }
            }
        }

        Err(Error::MaxRetries(MAX_RETRIES))
    }

    fn try_recv_packet(&self, buf: &mut [u8]) -> Result<usize, Error> {
        let header = self.read_header()?;
        if let Header::EndOfTransmission = header {
            return Ok(0);
        }

        let mut sum: usize = 0;
        for b in buf.iter_mut() {
            *b = self.0.recv_timeout()?;
            sum += *b as usize;
        }

        let checksum = self.0.recv_timeout()?;
        if checksum == (sum & 0xff) as u8 {
            Ok(buf.len())
        } else {
            Err(Error::ChecksumError)
        }
    }

    fn read_header(&self) -> Result<Header, Error> {
        let soh = self.0.recv_timeout()?;
        if soh == EOT {
            return Ok(Header::EndOfTransmission);
        }

        let block_num = self.0.recv_timeout()?;
        let inv_block_num = self.0.recv_timeout()?;
        if soh == SOH && 255 - block_num == inv_block_num {
            Ok(Header::BlockNumber(block_num))
        } else {
            Err(Error::InvalidHeader)
        }
    }
}
