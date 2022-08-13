
use yaml_rust::Yaml;
use std::error::Error;
use std::fmt;
use std::collections::HashMap;
use std::time::Duration;
use std::{net::UdpSocket};
use crate::sensors::SensorsProvider;
use super::providers::Provider;
const NAME: &str = "XPLN11";

const DATA_MESSAGE_MIN_BYTES_SIZE_VALUE:usize = 9;
const DATA_MESSAGE_READ_TIMEOUT_VALUE:Duration = Duration::from_millis(100);

#[derive(Debug)]
struct XPLN11Error(String);

impl fmt::Display for XPLN11Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for XPLN11Error {}

#[test]
fn load_test_data() {

    println!("load_test_data");

    let test_data_str = std::fs::read_to_string("src/providers/udp_fragment.txt").unwrap();
    let split = test_data_str.split(",");
    let vec: Vec<&str> = split.collect();
    let mut buf:[u8; 50] = [0; 50];

    let mut slice: &[u8];

    //for i in (0..buf.len()).step_by(1) {


    //let data: String = format!("{:?}", buf);

    //println!("test_data_str: {}, buf: {}, length: {}", test_data_str, data, vec.len());
}

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

pub struct XPLN11SensorsProvider {

    host: String,
    port: i64,
    socket: Option<Box<UdpSocket>>,
}

impl XPLN11SensorsProvider {

    fn build_new_data_message(&self, prologue: String) -> XPLN11UDPDataMessage {

        let data_message = XPLN11UDPDataMessage{
            prologue: prologue,
            data: HashMap::new()
        };

        data_message
    }

    fn connect(&mut self) -> std::io::Result<()> {
        println!("XXXXXX CONNECT BIND ---");
        let url = format!("{}:{}", self.host, self.port);
        let bind_result: Result<UdpSocket, std::io::Error> = UdpSocket::bind(url);
        
        if bind_result.is_ok() {
            println!("XXXXXX CONNECT OK ---");
            let s =  bind_result.unwrap();
            s.set_read_timeout(Some(DATA_MESSAGE_READ_TIMEOUT_VALUE)).expect("set_read_timeout call failed");

            self.socket = Some(Box::new(s));

            return Ok(());
        } else {
            let err = bind_result.unwrap_err();
            println!("XXXXXX Err --- {:?}", err);
            return Err(err);
        }
    }

    fn get_data(&self) -> Result<(), Box<dyn std::error::Error>> {

        let mut buf:[u8; 1000] = [0; 1000];
        let (number_of_bytes, _src) = self.socket.as_ref().unwrap().recv_from(&mut buf).unwrap();
        
        match self.decode_data(&number_of_bytes, buf) {
            Ok(message) => {

                let (_key, data_frg1) = message.data.get_key_value(&0).unwrap();
                println!("message index 0 data 1 : {}", data_frg1.data1);
                return Ok(())
            },
            Err(e) => Err(e)
        }

    }

    fn close(&self) {
        //NOPE
    }

    fn decode_data(&self, number_of_bytes: &usize, buf:[u8; 1000]) -> Result<XPLN11UDPDataMessage, Box<dyn Error>> {

        if *number_of_bytes < DATA_MESSAGE_MIN_BYTES_SIZE_VALUE {
            return Err(Box::new(XPLN11Error("Invalid message size".into())));
        }

        let mut data_message = self.build_new_data_message(String::from("data"));

        for i in (5..*number_of_bytes).step_by(4 + 32) {

            let index: i32 = self.decode_int_data(&buf, i);
            //println!("UDP index :'{}'", index);

            let data1: f32 = self.decode_float_data(&buf, i + 4);
            //println!("UDP data 1 :'{}'", data1);

            let data2: f32 = self.decode_float_data(&buf, i + 8);
            //println!("UDP data 2 :'{}'", data2);

            let data3: f32 = self.decode_float_data(&buf, i + 12);
            //println!("UDP data 3 :'{}'", data3);

            let data4: f32 = self.decode_float_data(&buf, i + 16);
            //println!("UDP data 4 :'{}'", data4);

            let data5: f32 = self.decode_float_data(&buf, i + 20);
            //println!("UDP data 5 :'{}'", data5);

            let data6: f32 = self.decode_float_data(&buf, i + 24);
            //println!("UDP data 6 :'{}'", data6);

            let data7: f32 = self.decode_float_data(&buf, i + 28);
            //println!("UDP data 7 :'{}'", data7);

            let data8: f32 = self.decode_float_data(&buf, i + 32);
            //println!("UDP data 8 :'{}'", data8);

            let data_fragment = XPLN11UDPDataFragment{index, data1, data2, data3, data4, data5, data6, data7, data8};
            
            data_message.data.insert(index, data_fragment);
        }

        Ok(data_message)
    }

    /// Decode i32 data from 4 bytes in little endian.
    fn decode_int_data(&self, buf: &[u8], start: usize) -> i32 {

        let mut bytes:[u8; 4] = [0;4];
        bytes[0] = buf[start];
        bytes[1] = buf[start + 1];
        bytes[2] = buf[start + 2];
        bytes[3] = buf[start + 3];

        let data_index = i32::from_le_bytes(bytes);

        return data_index;
    }

    // Decode f32 data from 4 bytes in little endian.
    fn decode_float_data(&self, buf: &[u8], start: usize) -> f32 {

        let mut bytes:[u8; 4] = [0;4];
        bytes[0] = buf[start];
        bytes[1] = buf[start + 1];
        bytes[2] = buf[start + 2];
        bytes[3] = buf[start + 3];

        let data_index = f32::from_le_bytes(bytes);

        return data_index;
    }

}

impl Provider for XPLN11SensorsProvider {

    fn name(&self) -> &str {

        return NAME;
    }

    fn init(&mut self) {
        println!("XPLN11 Provider config : host: {}, port: {}", self.host, self.port);
        self.connect().expect("Connection error");
        self.get_data().expect("Receive error");
    }

    fn shutdown(&self) {
        println!("XPLN11 SensorsProvider shutdown");
        self.close();
    }
}

impl SensorsProvider for XPLN11SensorsProvider {

    fn acquire(&self) {
        println!("XPLN11 SensorsProvider acquire");
        self.get_data();
    }
}

pub fn xpl11_provider_init(config: &Yaml) -> Box<dyn SensorsProvider> {

    println!("Start init xpl11_provider module");

    let host = config["provider"]["host"].as_str().unwrap().to_string();
    let port = config["provider"]["port"].as_i64().unwrap();

    let provider = Box::new(XPLN11SensorsProvider{host, port, socket:None});

    println!("End init xpl11_provider module");

    return provider;
}

