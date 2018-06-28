#![feature(const_string_new)]

use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::thread;
use std::env;
mod constants;

use constants::{STANDARD_COLOR, SUCCESS_COLOR, FAIL_COLOR, MING_DATA_COLOR, WALTON_DATA_COLOR};
use constants::{AMOUNT_GPU, PORT_NUMBER_START, HOST_ADDRESS, SERVER_ADDRESS};
use constants::{print_color, print_44, print_96, replace_nonce_random, print_args};

fn main() {
    print_color(&"Walton Proxy written in Rust".to_string(), &STANDARD_COLOR.to_owned());
    let args: Vec<_> = env::args().collect();
    // args: WaltonProxy.exe GPUs, PORT, SERVER, HOST
    if args.len() <= 1 {
        print_color("No arguments found, using default arguments", &FAIL_COLOR.to_owned());
        unsafe { print_args(&AMOUNT_GPU, &PORT_NUMBER_START, &SERVER_ADDRESS, &HOST_ADDRESS); }
    } else {
        unsafe {
            AMOUNT_GPU = args[1].parse().unwrap_or(1);
            PORT_NUMBER_START = args[2].parse().unwrap_or(12140);
            SERVER_ADDRESS = args[3].clone();
            HOST_ADDRESS = args[4].clone();
            print_args(&AMOUNT_GPU, &PORT_NUMBER_START, &SERVER_ADDRESS, &HOST_ADDRESS);
        }
    }
    thread::spawn(move || {
        unsafe { start_server("12125") };
    });
    unsafe { start_server("10241") };
}

unsafe fn start_server(port: &'static str) {
    let server_address = format!("{}:{}", SERVER_ADDRESS, port);
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

unsafe fn handle_client_ming(mut _walton_exe_socket: TcpStream) {
    _walton_exe_socket.set_nodelay(true).unwrap();
    //Init TcpStream Vector
    let mut _ming_socket_vector: Vec<TcpStream> = Vec::new();
    //Init TcpStreams depending on gpu amount
    for gpu in 0..AMOUNT_GPU {
        let port_number = PORT_NUMBER_START + gpu as i32;
        let host_address = format!("{}:{}", HOST_ADDRESS, port_number);
        let _ming_socket_temp = TcpStream::connect(&host_address)
            .expect("Please check the number of GPU's");
        _ming_socket_temp.set_nodelay(true).unwrap();
        _ming_socket_vector.push(_ming_socket_temp.try_clone().unwrap());
        print_color(&format!("Connected to ming_run.exe on port: {}", port_number),
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

unsafe fn handle_client_12125(mut _walton_exe_socket: TcpStream) {
    _walton_exe_socket.set_nodelay(true).unwrap();
    //Init TcpStream Vector
    let mut _ming_socket_vector: Vec<TcpStream> = Vec::new();
    //Init TcpStreams depending on gpu amount
    for gpu in 0..AMOUNT_GPU {
        let port_number = PORT_NUMBER_START + gpu as i32;
        let host_address = format!("{}:{}", HOST_ADDRESS, port_number);
        let _ming_socket_temp = TcpStream::connect(&host_address)
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
                for mut _msocket in &_ming_socket_vector {
                    let mut p: Vec<u8> = _packets_received.to_vec();
                    replace_nonce_random(&mut p);
//                            p[45]=200; //255 easiest difficulty
//                            p[0]=0; // set 0 or 1 for start new block
                    print_96(&p);
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

