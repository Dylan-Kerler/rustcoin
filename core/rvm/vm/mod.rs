pub mod instructions;

pub struct VM {
	sc: u32, //Stack pointer -- Program pointer can be omitted since non-turing complete
	STACK: Vec<u8>,
	BINARY_STORE: Vec<u8>, //Store for the bytecode
	MSTORE: Vec<u8>
}

impl VM {
	pub fn new(bytecode: Vec<u8>) -> VM {
		Self {
			sc: 0,
			STACK: Vec::new(),
			BINARY_STORE: bytecode,
			MSTORE: Vec::new()
		}
	}

	//Runs the bytecode of the tx
	pub fn execute(mut self) -> VM {
		let mut i: usize = 0;
		let mut STACK = self.STACK;
		let BINARY_STORE = self.BINARY_STORE;

		//loop through the BINARY_STORE and run the program
		while i < BINARY_STORE.len() {
			match BINARY_STORE[i] {
				START => {

				},
				PUSH => {
					i+=1;
					STACK.push(BINARY_STORE[i].clone());
				},
				GET_UTXO => {
					let tx_hash = STACK.pop();
					//TODO: Add tx index as well


				},
				DUP_HASH160 => {},
				EQUAL_VERIFY => {},
				CHECKSIG => {},
				END => {},
				_ => {}
			}
			i+=1;
		}
		println!("{:?}", STACK);
		self.BINARY_STORE = BINARY_STORE;
		self.STACK = STACK;
		return self;
	}
}