
use yaml_rust::Yaml;
use std::error::Error;
use std::sync::Arc;
use std::{fmt, thread};
use std::collections::HashMap;
use std::time::Duration;
use std::net::UdpSocket;
use crate::sensors::SensorsProvider;
use crate::flight_ctrl::FlightCtrlsProvider;
use crate::sensors::sensors::SensorsValues;
use super::providers::Provider;


const NAME: &str = "XPLN11";
const DATA_MESSAGE_BUFFER_SIZE_VALUE:usize = 1024;
const DATA_MESSAGE_MIN_BYTES_SIZE_VALUE:usize = 9;
const DATA_MESSAGE_READ_TIMEOUT_VALUE:Duration = Duration::from_millis(100);
const DATA_MESSAGE_WRITE_TIMEOUT_VALUE:Duration = Duration::from_millis(100);

#[derive(Debug)]
struct XPLN11Error(String);

impl fmt::Display for XPLN11Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for XPLN11Error {}

struct XPLN11UDPDataMessage {

    prologue: String,
    data: HashMap<i32, XPLN11UDPDataFragment> 
}

struct XPLN11UDPDataFragment {

    index: i32,
    data1: f32,
    data2: f32,
    data3: f32,
    data4: f32,
    data5: f32,
    data6: f32,
    data7: f32,
    data8: f32,
}

pub struct XPLN11Provider {

    host: String,
    read_port: i64,
    write_port: i64,
    socket: Option<UdpSocket>,
}

impl XPLN11Provider {

    fn connect_read_socket(&mut self) -> std::io::Result<()> {

        println!("Connecting to xplane read socket");

        let url: String = format!("{}:{}", self.host, self.read_port);

        let bind_result: Result<UdpSocket, std::io::Error> = UdpSocket::bind(url);
        
        if bind_result.is_ok() {

            println!("UDP read socket binded");
            let s: UdpSocket =  bind_result.unwrap();
            
            s.set_read_timeout(Some(DATA_MESSAGE_READ_TIMEOUT_VALUE)).expect("set_read_timeout call failed");
            s.set_write_timeout(Some(DATA_MESSAGE_WRITE_TIMEOUT_VALUE)).expect("set_write_timeout call failed");

            self.socket = Some(s);

            return Ok(());
        } else {
            let err = bind_result.unwrap_err();
            println!("Error during connection to xplane read socket : {:?}", err);
            return Err(err);
        }
    }

}

impl Provider for XPLN11Provider {

    fn name(&self) -> &str {

        return NAME;
    }

    fn init(&mut self) {
        println!("XPLN11 Provider config : host: {}, read_port: {}, write_port: {}", self.host, self.read_port, self.write_port);
        
        self.connect_read_socket().expect("Connection to read socket error");
    }

    fn shutdown(&self) {
        println!("XPLN11 Provider shutdown");
    }

    fn get_sensors(&self) -> Arc::<dyn SensorsProvider + Send + Sync> {

        let s: &UdpSocket = self.socket.as_ref().unwrap();

        let socket = s.try_clone().unwrap();
        Arc::new(XMPL11SensorsProvider{ socket: socket })
    }

    fn get_flcs(&self) -> Arc::<dyn FlightCtrlsProvider + Send + Sync> {

        let s: &UdpSocket = self.socket.as_ref().unwrap();

        let socket = s.try_clone().unwrap();
        Arc::new(XPLN11FlightCtrlsProvider{host: self.host.clone(), write_port: self.write_port, socket: socket})
    }
}

struct XMPL11SensorsProvider {

    socket: UdpSocket,
}

impl XMPL11SensorsProvider {

    fn get_data(&self) -> Result<XPLN11UDPDataMessage, Box<dyn std::error::Error>> {

        let mut buf:[u8; DATA_MESSAGE_BUFFER_SIZE_VALUE] = [0; DATA_MESSAGE_BUFFER_SIZE_VALUE];
        let socket: &UdpSocket = &self.socket;
        let (number_of_bytes, _src) = socket.recv_from(&mut buf).unwrap();
        
        match decode_data(&number_of_bytes, buf) {
            Ok(message) => {

                let (_key, data_frg1) = message.data.get_key_value(&0).unwrap();
                println!("message index 0 data 1 : {}", data_frg1.data1);
                return Ok(message)
            },
            Err(e) => return Err(e)
        }
    }
}

impl SensorsProvider for XMPL11SensorsProvider {

    fn acquire(&self) -> SensorsValues {
        println!("XPLN11 Provider acquire");
        let _raw_data = self.get_data();

        SensorsValues::new()
    }
}

struct  XPLN11FlightCtrlsProvider {

    host: String,
    write_port: i64,
    socket: UdpSocket
}


impl FlightCtrlsProvider for XPLN11FlightCtrlsProvider {

    fn send(&self) {

        let url = format!("{}:{}", self.host, self.write_port);
        let socket: &UdpSocket = &self.socket;
        //socket.connect(url).expect("Error connection to write socket");

        //while (true) {
            
        /* 
        let prologue: [u8; 4] = "DATA".as_bytes().try_into().unwrap();
        let space: [u8; 1] = 0u8.to_le_bytes(); //.as_bytes().try_into().unwrap();
        let index: [u8; 4] = 8i32.to_le_bytes();
        let data1: [u8; 4] = (-999f32).to_le_bytes();
        let data2: [u8; 4] = (-999f32).to_le_bytes();
        let data3: [u8; 4] = (-999f32).to_le_bytes();
        let data4: [u8; 4] = (-999f32).to_le_bytes();
        let data5: [u8; 4] = (-999f32).to_le_bytes();
        let data6: [u8; 4] = (-999f32).to_le_bytes();
        let data7: [u8; 4] = (-999f32).to_le_bytes();
        let data8: [u8; 4] = (-999f32).to_le_bytes();
        

        let mut vec: Vec<u8> = Vec::new();
        vec.extend_from_slice(&prologue);
        vec.extend_from_slice(&space);
        vec.extend_from_slice(&index);
        vec.extend_from_slice(&data1);
        vec.extend_from_slice(&data2);
        vec.extend_from_slice(&data3);
        vec.extend_from_slice(&data4);
        vec.extend_from_slice(&data5);
        vec.extend_from_slice(&data6);
        vec.extend_from_slice(&data7);
        vec.extend_from_slice(&data8);
        */

        let prologue: &[u8] = "DREF0".as_bytes();
        let value: [u8; 4] = 1f32.to_le_bytes();
        let dref: &[u8] = "sim/joystick/yoke_pitch_ratio".as_bytes();

        let mut vec: Vec<u8> = Vec::with_capacity(509);
        vec.extend_from_slice(&prologue);
        //vec.extend_from_slice(&sep);
        vec.extend_from_slice(&value);
        vec.extend_from_slice(dref);
        //vec.extend_from_slice(&sep);
        

        let message_len: usize = &prologue.len() + value.len() + dref.len();
        let required_message_len: usize = 509;
        let diff_len: usize = required_message_len - message_len;

        for _n in 0..diff_len {
            vec.push(0);
        }


        let message: &[u8] = &(vec)[..];

        println!("message len : {}", message.len());

        //while(true) {
        socket.send_to(&message, url).expect("Message send error");


        println!("Message sent !{:?}", message);
        //    thread::sleep(Duration::from_millis(100));
        //}
    }
}

pub fn xpl11_provider_init(config: &Yaml) -> Box<dyn Provider> {

    println!("Start init xpl11_provider module");

    let host = config["provider"]["host"].as_str().unwrap().to_string();
    let read_port = config["provider"]["read_port"].as_i64().unwrap();
    let write_port = config["provider"]["write_port"].as_i64().unwrap();

    let provider = Box::new(XPLN11Provider{ 
        host, 
        read_port, 
        write_port,
        socket: None,
    });

    println!("End init xpl11_provider module");

    return provider;
}

fn build_new_data_message(prologue: String) -> XPLN11UDPDataMessage {

    let data_message = XPLN11UDPDataMessage{
        prologue: prologue,
        data: HashMap::new()
    };

    data_message
}

fn decode_data(number_of_bytes: &usize, buf:[u8; DATA_MESSAGE_BUFFER_SIZE_VALUE]) -> Result<XPLN11UDPDataMessage, Box<dyn Error>> {
    
    if *number_of_bytes < DATA_MESSAGE_MIN_BYTES_SIZE_VALUE {
        return Err(Box::new(XPLN11Error("Invalid message size".into())));
    }

    let mut data_message = build_new_data_message(String::from("DATA"));

    for i in (5..*number_of_bytes).step_by(4 + 32) {

        let index: i32 = decode_int_data(&buf, i);
        //println!("UDP index :'{}'", index);

        let data1: f32 = decode_float_data(&buf, i + 4);
        //println!("UDP data 1 :'{}'", data1);

        let data2: f32 = decode_float_data(&buf, i + 8);
        //println!("UDP data 2 :'{}'", data2);

        let data3: f32 = decode_float_data(&buf, i + 12);
        //println!("UDP data 3 :'{}'", data3);

        let data4: f32 = decode_float_data(&buf, i + 16);
        //println!("UDP data 4 :'{}'", data4);

        let data5: f32 = decode_float_data(&buf, i + 20);
        //println!("UDP data 5 :'{}'", data5);

        let data6: f32 = decode_float_data(&buf, i + 24);
        //println!("UDP data 6 :'{}'", data6);

        let data7: f32 = decode_float_data(&buf, i + 28);
        //println!("UDP data 7 :'{}'", data7);

        let data8: f32 = decode_float_data(&buf, i + 32);
        //println!("UDP data 8 :'{}'", data8);

        let data_fragment = XPLN11UDPDataFragment{index, data1, data2, data3, data4, data5, data6, data7, data8};
        
        data_message.data.insert(index, data_fragment);
    }

    Ok(data_message)
}

/// Decode i32 data from 4 bytes in little endian.
fn decode_int_data(buf: &[u8], start: usize) -> i32 {

    let mut bytes:[u8; 4] = [0;4];
    bytes[0] = buf[start];
    bytes[1] = buf[start + 1];
    bytes[2] = buf[start + 2];
    bytes[3] = buf[start + 3];

    let data_index = i32::from_le_bytes(bytes);

    return data_index;
}

// Decode f32 data from 4 bytes in little endian.
fn decode_float_data(buf: &[u8], start: usize) -> f32 {

    let mut bytes:[u8; 4] = [0;4];
    bytes[0] = buf[start];
    bytes[1] = buf[start + 1];
    bytes[2] = buf[start + 2];
    bytes[3] = buf[start + 3];

    let data_index = f32::from_le_bytes(bytes);

    return data_index;
}

#[test]
fn load_test_data() {

    println!("load_test_data");

    let test_data_str = std::fs::read_to_string("src/providers/udp_fragment.txt").unwrap();
    let split = test_data_str.split(",");
    let vec: Vec<&str> = split.collect();
    let vec2: &Vec<u8> = &vec.iter().map(|c| c.parse::<u8>().unwrap()).collect();

    let mut buf:[u8; DATA_MESSAGE_BUFFER_SIZE_VALUE] = [0; DATA_MESSAGE_BUFFER_SIZE_VALUE];

    let number_of_bytes: usize = vec2.len();

    for i in 0..vec.len() {

        buf[i] = vec2[i];
    }

    let decoded_data: XPLN11UDPDataMessage = decode_data(&number_of_bytes, buf).unwrap();

    assert_eq!("DATA", decoded_data.prologue);
    assert_eq!(1, decoded_data.data.len());

    let decoded_frag: &XPLN11UDPDataFragment = decoded_data.data.get(&0).unwrap();

    println!("frag index : {}", decoded_frag.index);
    assert_eq!(0, decoded_frag.index);

    println!("frag index : {}, data 1 : {}", decoded_frag.index, decoded_frag.data1);
    assert_eq!(23.095982, decoded_frag.data1);

    println!("frag index : {}, data 2 : {}", decoded_frag.index, decoded_frag.data2);
    assert_eq!(21.352922, decoded_frag.data2);

    println!("frag index : {}, data 3 : {}", decoded_frag.index, decoded_frag.data3);
    assert_eq!(-999f32, decoded_frag.data3);

    println!("frag index : {}, data 4 : {}", decoded_frag.index, decoded_frag.data4);
    assert_eq!(0.043297574, decoded_frag.data4);

    println!("frag index : {}, data 5 : {}", decoded_frag.index, decoded_frag.data5);
    assert_eq!(0.020787029, decoded_frag.data5);

    println!("frag index : {}, data 6 : {}", decoded_frag.index, decoded_frag.data6);
    assert_eq!(0.024036264, decoded_frag.data6);

    println!("frag index : {}, data 7 : {}", decoded_frag.index, decoded_frag.data7);
    assert_eq!(1f32, decoded_frag.data7);

    println!("frag index : {}, data 8 : {}", decoded_frag.index, decoded_frag.data8);
    assert_eq!(1f32, decoded_frag.data8);

}
