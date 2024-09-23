use std::{
    fmt::Display, io::{Read, Write}, net::{TcpStream, ToSocketAddrs}
};

// python example
//
// class ATIController(object):
//     def __init__(self, ip='192.168.1.1'):
//         self.__control = socket.socket()
//         self.__control.connect((ip, 49151))
//         # read ATI sensor upon command
//         self.__read_calibration_info = bytes([0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//                                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
//         self.__read_force = bytes([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//                             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
//         self.__reset_force = bytes([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//                              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01])
//         self.__countsPerForce = 1000000
//         self.__countsPerTorque = 1000000
//         self.__scaleFactors_force = 15260
//         self.__scaleFactors_torque = 92

//     def setZero(self):
//         self.__control.send(self.__reset_force)

//     def readDate(self):
//         self.__control.send(self.__read_force)
//         force_info = self.__control.recv(16)
//         header, status, ForceX, ForceY, ForceZ, TorqueX, TorqueY, TorqueZ = struct.unpack('!2H6h', force_info)
//         Fx = ForceX * self.__scaleFactors_force / self.__countsPerForce
//         Fy = ForceY * self.__scaleFactors_force / self.__countsPerForce
//         Fz = ForceZ * self.__scaleFactors_force / self.__countsPerForce
//         Tx = TorqueX * self.__scaleFactors_torque / self.__countsPerTorque
//         Ty = TorqueY * self.__scaleFactors_torque / self.__countsPerTorque
//         Tz = TorqueZ * self.__scaleFactors_torque / self.__countsPerTorque
//         # N, NM
//         Force_torque = np.array([Fx, Fy, Fz, Tx, Ty, Tz])
//         return Force_torque

pub struct AtiNano25 {
    connect: TcpStream,
    buffer: [u8; 32],
    scale_factors_force: f64,
    scale_factors_torque: f64,
    counts_per_force: f64,
    counts_per_torque: f64,
}

#[derive(Debug)]
pub struct DataFrame {
    pub header: u16,
    pub status: u16,
    pub force_x: f64,
    pub force_y: f64,
    pub force_z: f64,
    pub torque_x: f64,
    pub torque_y: f64,
    pub torque_z: f64,
}

impl Display for DataFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "force_x: {:.5} N,\nforce_y: {:.5} N,\nforce_z: {:.5} N,\ntorque_x: {:.5} Nm,\ntorque_y: {:.5} Nm,\ntorque_z: {:.5} Nm\n",
            self.force_x, self.force_y, self.force_z, self.torque_x, self.torque_y, self.torque_z
        )
    }
}

impl AtiNano25 {
    pub fn new<A: ToSocketAddrs>(addr: A) -> AtiNano25 {
        let connect = TcpStream::connect(addr).unwrap();
        AtiNano25 {
            connect,
            buffer: [0; 32],
            scale_factors_force: 15260.,
            scale_factors_torque: 92.,
            counts_per_force: 1000000.,
            counts_per_torque: 1000000.,
        }
    }

    // fn read_calibration_info(&mut self) {
    //     let read_calibration_info = [
    //         0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     ];
    //     self.connect.write_all(&read_calibration_info).unwrap();
    //     self.connect.flush().unwrap();
    //     let len = self.connect.read(&mut self.buffer).unwrap();
    //     println!("{:?}", &self.buffer[..len]);
    // }

    pub fn set_zero(&mut self) {
        let reset_force = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        ];
        self.connect.write_all(&reset_force).unwrap();
        self.connect.flush().unwrap();
    }

    pub fn read_force(&mut self) -> DataFrame {
        let read_force = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        self.connect.write_all(&read_force).unwrap();
        self.connect.flush().unwrap();
        let _len = self.connect.read(&mut self.buffer).unwrap();

        let header = u16::from_be_bytes([self.buffer[0], self.buffer[1]]);
        let status = u16::from_be_bytes([self.buffer[2], self.buffer[3]]);
        let force_x = i16::from_be_bytes([self.buffer[4], self.buffer[5]]) as f64;
        let force_y = i16::from_be_bytes([self.buffer[6], self.buffer[7]]) as f64;
        let force_z = i16::from_be_bytes([self.buffer[8], self.buffer[9]]) as f64;
        let torque_x = i16::from_be_bytes([self.buffer[10], self.buffer[11]]) as f64;
        let torque_y = i16::from_be_bytes([self.buffer[12], self.buffer[13]]) as f64;
        let torque_z = i16::from_be_bytes([self.buffer[14], self.buffer[15]]) as f64;
        DataFrame {
            header,
            status,
            force_x: force_x * self.scale_factors_force / self.counts_per_force,
            force_y: force_y * self.scale_factors_force / self.counts_per_force,
            force_z: force_z * self.scale_factors_force / self.counts_per_force,
            torque_x: torque_x * self.scale_factors_torque / self.counts_per_torque,
            torque_y: torque_y * self.scale_factors_torque / self.counts_per_torque,
            torque_z: torque_z * self.scale_factors_torque / self.counts_per_torque,
        }
    }
}