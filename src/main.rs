use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::thread;
mod constants;
use constants::{STANDARD_COLOR,SUCCESS_COLOR,FAIL_COLOR,MING_DATA_COLOR,PORT_NUMBER_VECTOR,WALTON_DATA_COLOR,AMOUNT_GPU};
use constants::{print_color, print_block};

fn main() {
    print_color(&"Walton Proxy written in Rust".to_string(), &STANDARD_COLOR.to_owned());
    thread::spawn(|| {
        start_server_12125();
    });
    start_server_10241();
}

fn start_server_10241(){
    let server_address = "127.0.0.1:10241";
    let listener = TcpListener::bind(&server_address).unwrap();
    print_color(&format!("Server is listening on {}", &server_address), &MING_DATA_COLOR.to_owned());
    print_color("Waiting for a connection..", &MING_DATA_COLOR.to_owned());
    for stream in listener.incoming() {
        match stream {
            Ok(_walton_exe_socket) => {
                thread::spawn(|| {
                    handle_client_10241(_walton_exe_socket);
                });
            },
            Err(e) => {
                print_color(&format!("Error occurred {}", e),&FAIL_COLOR.to_owned());
                panic!("");
            },
        }
    }
}

fn start_server_12125(){
    let server_address = "127.0.0.1:12125";
    let listener = TcpListener::bind(&server_address).unwrap();
    print_color(&format!("Server is listening on {}", &server_address), &WALTON_DATA_COLOR.to_owned());
    print_color("Waiting for a connection..", &WALTON_DATA_COLOR.to_owned());
    for stream in listener.incoming() {
        match stream {
            Ok(_walton_exe_socket) => {
                thread::spawn(|| {
                    handle_client_12125(_walton_exe_socket);
                });
            },
            Err(e) => {
                print_color(&format!("Error occured {}", e),&FAIL_COLOR.to_owned());
                panic!("");
            },
        }
    }
}

fn handle_client_10241(mut _walton_exe_socket: TcpStream){
    _walton_exe_socket.set_nodelay(true).unwrap();
    //Init first TcpStream
    let mut _ming_socket_vector: Vec<TcpStream> = Vec::new();
    let mut _ming_socket = TcpStream::connect(format!("127.0.0.1:{}",PORT_NUMBER_VECTOR[0])).unwrap();
    _ming_socket.set_nodelay(true).unwrap();
    print_color(&format!("Connected to ming_run.exe on port: {}",PORT_NUMBER_VECTOR[0]), &MING_DATA_COLOR.to_owned());
    _ming_socket_vector.push(_ming_socket.try_clone().unwrap());
    //Init other TcpStreams depending on gpu amount
    for gpu in 1..*AMOUNT_GPU{
        let index = gpu as usize;
        let _ming_socket_temp = TcpStream::connect(format!("127.0.0.1:{}",PORT_NUMBER_VECTOR[index])).unwrap();
        _ming_socket_temp.set_nodelay(true).unwrap();
        _ming_socket_vector.push(_ming_socket_temp.try_clone().unwrap());
        print_color(&format!("Connected to ming_run.exe on port: {}",PORT_NUMBER_VECTOR[index]), &MING_DATA_COLOR.to_owned());
    }
        let mut packets_received_socket = [0;100];
        let  _walton_result = _walton_exe_socket.read(&mut packets_received_socket);
        match  _walton_result{
            Ok(walton_exe_socket) => {
                if walton_exe_socket>0{
                    print_color(&format!("Received {} bytes from ming_run.exe on port 10241", walton_exe_socket),&MING_DATA_COLOR.to_owned());
                    let _packets_received = &packets_received_socket.get(0..walton_exe_socket).unwrap();
                    print_color(&format!("Receiving: {:?}", _packets_received.to_vec()),&MING_DATA_COLOR.to_owned());
                    print_44(&_packets_received.to_vec());
                    for mut _msocket in &_ming_socket_vector{
                        _msocket.write_all(&_packets_received).unwrap();
                        print_color("Sent bytes to walton.exe ",&MING_DATA_COLOR.to_owned());
                        _msocket.shutdown(Shutdown::Both).expect("Shutdown call failed");
                    }
                }
            }
            Err(e) => {
                print_color(&format!("Error occurred {}", e),&FAIL_COLOR.to_owned());
                panic!("");
            }
        }
}

fn handle_client_12125(mut _walton_exe_socket: TcpStream){
    _walton_exe_socket.set_nodelay(true).unwrap();
    //Init first TcpStream
    let mut _ming_socket_vector: Vec<TcpStream> = Vec::new();
    let mut _ming_socket = TcpStream::connect(format!("127.0.0.1:{}",PORT_NUMBER_VECTOR[0])).unwrap();
    _ming_socket.set_nodelay(true).unwrap();
    print_color(&format!("Connected to ming_run.exe on port: {}",PORT_NUMBER_VECTOR[0]), &WALTON_DATA_COLOR.to_owned());
    _ming_socket_vector.push(_ming_socket.try_clone().unwrap());
    //Init other TcpStreams depending on gpu amount
    for gpu in 1..*AMOUNT_GPU{
        let index = gpu as usize;
        let _ming_socket_temp = TcpStream::connect(format!("127.0.0.1:{}",PORT_NUMBER_VECTOR[index])).unwrap();
        _ming_socket_temp.set_nodelay(true).unwrap();
        _ming_socket_vector.push(_ming_socket_temp.try_clone().unwrap());
        print_color(&format!("Connected to ming_run.exe on port: {}",PORT_NUMBER_VECTOR[index]), &WALTON_DATA_COLOR.to_owned());
    }
            let mut packets_received_socket = [0;100];
            let  _walton_result = _walton_exe_socket.read(&mut packets_received_socket);
            match  _walton_result{
                Ok(walton_exe_socket) => {
                    if walton_exe_socket>0{
                        //send to all ming_run.exe files
                        print_color(&format!("Received {} bytes from walton.exe", walton_exe_socket),&WALTON_DATA_COLOR.to_owned());
                        let _packets_received = &packets_received_socket.get(0..walton_exe_socket).unwrap();
                        print_color(&format!("Receiving: {:?}", _packets_received.to_vec()),&WALTON_DATA_COLOR.to_owned());
                        print_96(&_packets_received.to_vec());
                        for mut _msocket in &_ming_socket_vector{
                            let mut p:Vec<u8> = _packets_received.to_vec(); //Replace Difficulty
//                            p[45]=200; //255 easiest difficulty
//                            p[0]=0; // set 0 or 1 for start new block
                            _msocket.write_all(&p).unwrap();
                            print_color(&format!("Sent bytes to ming_run.exe running on port: {}",  _msocket.peer_addr().unwrap()),&WALTON_DATA_COLOR.to_owned());
                            _msocket.shutdown(Shutdown::Both).expect("Shutdown call failed");
                        }
                    }
                }
                Err(e) => {
                    print_color(&format!("Error occured {}", e),&FAIL_COLOR.to_owned());
                    panic!("");
                }
            }
}

fn print_44(packets: &Vec<u8>){
    print_color("Formatted:",&MING_DATA_COLOR.to_owned());
    let unidentified_1 = packets.get(0..2).unwrap();
    let unidentified_2 = packets.get(2..4).unwrap();
    let input = packets.get(4..36).unwrap();
    let input_nonce = packets.get(36..44).unwrap();
    print_color(&format!("  Unidentified_1: {:?}",&unidentified_1.to_vec()),&STANDARD_COLOR.to_owned());
    print_color(&format!("  Unidentified_2: {:?}",&unidentified_2.to_vec()),&STANDARD_COLOR.to_owned());
    print_color(&format!("  Input Value   : {:?}",&input.to_vec()),&STANDARD_COLOR.to_owned());
    print_color(&format!("  Input Nonce   : {:?}",&input_nonce.to_vec()),&STANDARD_COLOR.to_owned());
}

fn print_96(packets: &Vec<u8>){
    print_color("Formatted:",&WALTON_DATA_COLOR.to_owned());
    let block_number = packets.get(1..5).unwrap();  //Byte Index 1-4 - Index 0 = set/stop
    let count = packets.get(77..85).unwrap(); // Count is constant
    let input_nonce = packets.get(37..45).unwrap();
    let algtion_val = packets.get(85..96).unwrap();
    let input_val = packets.get(5..37).unwrap();
    let target_val = packets.get(45..77).unwrap();
    print_block(&block_number);
    print_color(&format!("  Count Val  : {:?}",&count.to_vec()),&SUCCESS_COLOR.to_owned());
    print_color(&format!("  Input Nonce: {:?}",&input_nonce.to_vec()),&SUCCESS_COLOR.to_owned());
    print_color(&format!("  Algtion Val: {:?}",&algtion_val.to_vec()),&SUCCESS_COLOR.to_owned());
    print_color(&format!("  Input   Val: {:?}",&input_val.to_vec()),&SUCCESS_COLOR.to_owned());
    print_color(&format!("  Target  Val: {:?}",&target_val.to_vec()),&SUCCESS_COLOR.to_owned());
}

