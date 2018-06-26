use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::thread;
mod constants;

use constants::{STANDARD_COLOR, SUCCESS_COLOR, FAIL_COLOR, MING_DATA_COLOR, WALTON_DATA_COLOR};
use constants::{AMOUNT_GPU, PORT_NUMBER_START};
use constants::{print_color, print_44, print_96};

fn main() {
    print_color(&"Walton Proxy written in Rust".to_string(), &STANDARD_COLOR.to_owned());
    thread::spawn(|| {
        start_server("127.0.0.1", "12125");
    });
    start_server("127.0.0.1", "10241");
}

fn start_server(address: &str, port: &'static str) {
    let server_address = format!("{}:{}", address, port);
    let listener = TcpListener::bind(&server_address).unwrap();
    print_color(&format!("Server is listening on {}", &server_address), &SUCCESS_COLOR.to_owned());
    print_color("Waiting for a connection..", &SUCCESS_COLOR.to_owned());
    for stream in listener.incoming() {
        match stream {
            Ok(_walton_exe_socket) => {
                thread::spawn(move || {
                    if port == "12125" {
                        handle_client_12125(_walton_exe_socket);
                    } else {
                        handle_client_ming(_walton_exe_socket);
                    }
                });
            },
            Err(e) => {
                print_color(&format!("Error occurred {}", e), &FAIL_COLOR.to_owned());
                panic!("");
            },
        }
    }
}

fn handle_client_ming(mut _walton_exe_socket: TcpStream) {
    _walton_exe_socket.set_nodelay(true).unwrap();
    //Init TcpStream Vector
    let mut _ming_socket_vector: Vec<TcpStream> = Vec::new();
    //Init TcpStreams depending on gpu amount
    for gpu in 0..*AMOUNT_GPU {
        let port_number = PORT_NUMBER_START + gpu as i32;
        let _ming_socket_temp = TcpStream::connect(format!("127.0.0.1:{}", port_number))
            .expect("Please check the number of GPU's");
        _ming_socket_temp.set_nodelay(true).unwrap();
        _ming_socket_vector.push(_ming_socket_temp.try_clone().unwrap());
        print_color(&format!("Connected to ming_run.exe on port: {}", 1),
                    &MING_DATA_COLOR.to_owned());
    }
    let mut packets_received_socket = [0; 100];
    let _walton_result = _walton_exe_socket.read(&mut packets_received_socket);
    match _walton_result {
        Ok(walton_exe_socket) => {
            if walton_exe_socket > 0 {
                print_color(&format!("Received {} bytes from ming_run.exe on port 10241", walton_exe_socket),
                            &MING_DATA_COLOR.to_owned());
                let _packets_received = &packets_received_socket.get(0..walton_exe_socket).unwrap();
                print_color(&format!("Receiving: {:?}", _packets_received.to_vec()),
                            &MING_DATA_COLOR.to_owned());
                print_44(&_packets_received.to_vec());
                for mut _msocket in &_ming_socket_vector {
                    _msocket.write_all(&_packets_received).unwrap();
                    print_color("Sent bytes to walton.exe ",
                                &MING_DATA_COLOR.to_owned());
                    _msocket.shutdown(Shutdown::Both).expect("Shutdown call failed");
                }
            }
        }
        Err(e) => {
            print_color(&format!("Error occurred {}", e),
                        &FAIL_COLOR.to_owned());
            panic!();
        }
    }
}

fn handle_client_12125(mut _walton_exe_socket: TcpStream){
    _walton_exe_socket.set_nodelay(true).unwrap();
    //Init TcpStream Vector
    let mut _ming_socket_vector: Vec<TcpStream> = Vec::new();
    //Init TcpStreams depending on gpu amount
    for gpu in 0..*AMOUNT_GPU {
        let port_number = PORT_NUMBER_START + gpu as i32;
        let _ming_socket_temp = TcpStream::connect(format!("127.0.0.1:{}", port_number))
            .expect("Please check the number of GPU's");
        _ming_socket_temp.set_nodelay(true).unwrap();
        _ming_socket_vector.push(_ming_socket_temp.try_clone().unwrap());
        print_color(&format!("Connected to ming_run.exe on port: {}", port_number),
                    &WALTON_DATA_COLOR.to_owned());
    }
    let mut packets_received_socket = [0; 100];
    let _walton_result = _walton_exe_socket.read(&mut packets_received_socket);
    match _walton_result {
        Ok(walton_exe_socket) => {
            if walton_exe_socket > 0 {
                //send to all ming_run.exe files
                print_color(&format!("Received {} bytes from walton.exe", walton_exe_socket),
                            &WALTON_DATA_COLOR.to_owned());
                let _packets_received = &packets_received_socket.get(0..walton_exe_socket).unwrap();
                print_color(&format!("Receiving: {:?}", _packets_received.to_vec()),
                            &WALTON_DATA_COLOR.to_owned());
                print_96(&_packets_received.to_vec());
                for mut _msocket in &_ming_socket_vector {
                    let mut p: Vec<u8> = _packets_received.to_vec(); //Replace Difficulty
//                            p[45]=200; //255 easiest difficulty
//                            p[0]=0; // set 0 or 1 for start new block
                    _msocket.write_all(&p).unwrap();
                    print_color(&format!("Sent bytes to ming_run.exe running on port: {}", _msocket.peer_addr().unwrap()),
                                &WALTON_DATA_COLOR.to_owned());
                    _msocket.shutdown(Shutdown::Both).expect("Shutdown call failed");
                }
            }
        }
        Err(e) => {
            print_color(&format!("Error occured {}", e), &FAIL_COLOR.to_owned());
            panic!();
        }
    }
}

