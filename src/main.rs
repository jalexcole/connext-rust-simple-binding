#[repr(C)]
#[allow(non_camel_case_types)]
struct DDS_DomainParticipantFactory {
    _private: [u8; 0],
}
#[repr(C)]
#[allow(non_camel_case_types)]
struct DDS_DomainParticipant {
    _private: [u8; 0],
}
#[repr(C)]
#[allow(non_camel_case_types)]
struct DDS_DataWriter {
    _private: [u8; 0],
}
#[repr(C)]
#[allow(non_camel_case_types)]
struct HelloWorldDataWriter {
    _private: [u8; 0],
}
#[repr(C)]
#[allow(non_camel_case_types)]
struct DDS_InstanceHandle_t {
    _private: [u8; 0],
}
#[repr(C)]
#[allow(non_camel_case_types)]
struct DDS_DataReader {
    _private: [u8; 0],
}
#[repr(C)]
#[allow(non_camel_case_types)]
struct HelloWorldDataReader {
    _private: [u8; 0],
}
#[repr(C)]
#[allow(non_camel_case_types)]
struct DDS_SampleInfo {
    _private: [u8; 0],
}
#[repr(C)]
#[allow(non_camel_case_types)]
struct HelloWorld {
    msg: *mut i8,
}

const DDS_RETCODE_OK: i32 = 0;

extern "C" {
    fn DDS_DomainParticipantFactory_get_instance() -> *mut DDS_DomainParticipantFactory;
    fn HelloWorldTypeSupport_register_type(
        participant: *mut DDS_DomainParticipant,
        type_name: *const i8,
    ) -> i32;
    fn DDS_DomainParticipantFactory_register_type_support(
        DDS_DomainParticipantFactory: *mut DDS_DomainParticipantFactory,
        DDS_DomainParticipantFactory_RegisterTypeFunction: unsafe extern "C" fn(
            *mut DDS_DomainParticipant,
            *const i8,
        ) -> i32,
        type_name: *const i8,
    ) -> i32;
    fn DDS_DomainParticipantFactory_create_participant_from_config(
        factory: *mut DDS_DomainParticipantFactory,
        configuration_name: *const i8,
    ) -> *mut DDS_DomainParticipant;
    fn DDS_DomainParticipant_lookup_datawriter_by_name(
        participant: *mut DDS_DomainParticipant,
        datawriter_full_name: *const i8,
    ) -> *mut DDS_DataWriter;
    fn HelloWorldDataWriter_narrow(writer: *mut DDS_DataWriter) -> *mut HelloWorldDataWriter;
    fn HelloWorldTypeSupport_create_data() -> *mut HelloWorld;
    fn HelloWorldDataWriter_write(
        writer: *mut HelloWorldDataWriter,
        instance_data: *mut HelloWorld,
        handle: *const DDS_InstanceHandle_t,
    ) -> i32;
    fn HelloWorldTypeSupport_delete_data(sample: *mut HelloWorld) -> i32;

    fn DDS_DomainParticipant_lookup_datareader_by_name(
        participant: *mut DDS_DomainParticipant,
        datareader_full_name: *const i8,
    ) -> *mut DDS_DataReader;
    fn HelloWorldDataReader_narrow(writer: *mut DDS_DataReader) -> *mut HelloWorldDataReader;
    fn HelloWorldDataReader_take_next_sample(
        reader: *mut HelloWorldDataReader,
        received_data: *mut HelloWorld,
        sample_info: *mut DDS_SampleInfo,
    ) -> i32;
    fn DDS_DomainParticipant_delete_contained_entities(
        participant: *mut DDS_DomainParticipant,
    ) -> i32;
    fn DDS_DomainParticipantFactory_delete_participant(
        factory: *mut DDS_DomainParticipantFactory,
        participant: *mut DDS_DomainParticipant,
    ) -> i32;
    fn DDS_DomainParticipantFactory_finalize_instance() -> i32;
    static DDS_HANDLE_NIL: DDS_InstanceHandle_t;
}

#[derive(Debug)]
enum Mode {
    Publisher,
    Subscriber,
}

fn usage(appname: &str) {
    eprintln!("Usage: {appname} <pub|sub>");
}

fn main() -> Result<(), String> {
    let mode = {
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            usage(&args[0]);
            return Err("Invalid arguments".to_string());
        }
        match args[1].as_str() {
            "pub" => Mode::Publisher,
            "sub" => Mode::Subscriber,
            _ => {
                usage(&args[0]);
                return Err("Invalid mode".to_string());
            }
        }
    };

    let factory = unsafe { DDS_DomainParticipantFactory_get_instance() };
    if factory.is_null() {
        return Err("Failed to get DomainParticipantFactory instance".to_string());
    }

    let type_name = std::ffi::CString::new("HelloWorld").unwrap();
    let retcode = unsafe {
        DDS_DomainParticipantFactory_register_type_support(
            factory,
            HelloWorldTypeSupport_register_type,
            type_name.as_ptr(),
        )
    };
    if retcode != DDS_RETCODE_OK {
        return Err(format!("Failed to register type: {retcode}"));
    }

    let participant_name = std::ffi::CString::new("MyParticipantLibrary::MyParticipant").unwrap();
    let participant = unsafe {
        DDS_DomainParticipantFactory_create_participant_from_config(
            factory,
            participant_name.as_ptr(),
        )
    };
    if participant.is_null() {
        return Err("Failed to create DomainParticipant".to_string());
    }

    let helloworld = unsafe { HelloWorldTypeSupport_create_data() };
    if helloworld.is_null() {
        return Err("Failed to create Sample".to_string());
    }

    match mode {
        Mode::Publisher => {
            let datawriter_name = std::ffi::CString::new("MyPublisher::MyWriter").unwrap();
            let datawriter = unsafe {
                HelloWorldDataWriter_narrow(DDS_DomainParticipant_lookup_datawriter_by_name(
                    participant,
                    datawriter_name.as_ptr(),
                ))
            };
            if datawriter.is_null() {
                return Err("Failed to lookup DataWriter".to_string());
            }

            for idx in 1..=10 {
                let msg = format!("Hello world, I said {idx} times");
                for (i, &byte) in msg.as_bytes().iter().enumerate() {
                    unsafe {
                        *((*helloworld).msg.add(i)) = byte as i8;
                    }
                }

                let retcode =
                    unsafe { HelloWorldDataWriter_write(datawriter, helloworld, &DDS_HANDLE_NIL) };
                if retcode != DDS_RETCODE_OK {
                    return Err(format!("Failed to write sample: {retcode}"));
                }
                println!("Sample sent: {msg}");

                std::thread::sleep(std::time::Duration::from_secs(2));
            }
        }
        Mode::Subscriber => {
            let datareader_name = std::ffi::CString::new("MySubscriber::MyReader").unwrap();
            let datareader = unsafe {
                HelloWorldDataReader_narrow(DDS_DomainParticipant_lookup_datareader_by_name(
                    participant,
                    datareader_name.as_ptr(),
                ))
            };
            if datareader.is_null() {
                return Err("Failed to lookup DataReader".to_string());
            }

            loop {
                /* We dont want to replicate DDS_SampleInfo, lets reserve enough memory for it */
                let unused_sample_info = [0u8; 4096];
                let retcode = unsafe {
                    HelloWorldDataReader_take_next_sample(
                        datareader,
                        helloworld,
                        &unused_sample_info as *const u8 as *mut DDS_SampleInfo,
                    )
                };
                if retcode == DDS_RETCODE_OK {
                    let c_str = unsafe { std::ffi::CStr::from_ptr((*helloworld).msg) };
                    let str_slice = c_str.to_str().unwrap();
                    println!("Sample received: {}", str_slice);
                }
            }
        }
    }

    // Teardown
    {
        let retcode = unsafe { HelloWorldTypeSupport_delete_data(helloworld) };
        if retcode != DDS_RETCODE_OK {
            return Err(format!("Failed to delete sample: {retcode}"));
        }

        let retcode = unsafe { DDS_DomainParticipant_delete_contained_entities(participant) };
        if retcode != DDS_RETCODE_OK {
            return Err(format!("Failed to delete contained entities: {retcode}"));
        }

        let retcode =
            unsafe { DDS_DomainParticipantFactory_delete_participant(factory, participant) };
        if retcode != DDS_RETCODE_OK {
            return Err(format!("Failed to delete participant: {retcode}"));
        }

        let retcode = unsafe { DDS_DomainParticipantFactory_finalize_instance() };
        if retcode != DDS_RETCODE_OK {
            return Err(format!("Failed to finalize participant factory: {retcode}"));
        }
    }

    Ok(())
}
