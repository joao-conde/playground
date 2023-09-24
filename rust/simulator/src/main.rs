type Word = u8;
type Address = u32;

pub enum Mode {
    Kernel,
    User,
}

pub struct FlatMemory {
    memory: Vec<Word>,
}

pub trait Memory {
    fn read(&self, addr: Address) -> Option<Word>;
    fn write(&mut self, addr: Address, value: Word) -> bool;
    fn is_valid(&self, addr: Address) -> bool;
}

pub trait Protected {
    fn handle_mode<T, F: FnOnce(&mut ProtectedMemory) -> T, G: FnOnce(&mut ProtectedMemory) -> T>(
        &mut self,
        kernel_action: F,
        user_action: G,
    ) -> T;
}

pub trait PermissionMemory: Memory + Protected {
    fn can_read(&self, addr: Address) -> bool;
    fn can_write(&self, addr: Address) -> bool;
    fn can_execute(&self, addr: Address) -> bool;
}

pub trait Translated: Memory + Protected {
    fn translate(&self, virtual_address: Address) -> Option<Address>;
}

pub struct ProtectedMemory {
    flat_memory: FlatMemory,
    current_mode: Mode,
}

impl PermissionMemory for ProtectedMemory {
    fn can_read(&self, _addr: Address) -> bool {
        true
    }
    fn can_write(&self, _addr: Address) -> bool {
        true
    }
    fn can_execute(&self, _addr: Address) -> bool {
        true
    }
}

impl Memory for FlatMemory {
    fn read(&self, addr: Address) -> Option<Word> {
        if self.is_valid(addr) {
            Some(self.memory[addr as usize])
        } else {
            None
        }
    }

    fn write(&mut self, addr: Address, value: Word) -> bool {
        if self.is_valid(addr) {
            self.memory[addr as usize] = value;
            true
        } else {
            false
        }
    }

    fn is_valid(&self, addr: Address) -> bool {
        (addr as usize) < self.memory.len()
    }
}

impl Memory for ProtectedMemory {
    fn read(&self, addr: Address) -> Option<Word> {
        if self.can_read(addr) {
            let physical_address = self.translate(addr)?;
            self.flat_memory.read(physical_address)
        } else {
            None
        }
    }

    fn write(&mut self, addr: Address, value: Word) -> bool {
        self.can_write(addr)
            && self.translate(addr).map_or(false, |physical_address| {
                self.flat_memory.write(physical_address, value)
            })
    }

    fn is_valid(&self, addr: Address) -> bool {
        self.flat_memory.is_valid(addr)
    }
}

impl Translated for ProtectedMemory {
    fn translate(&self, virtual_address: Address) -> Option<Address> {
        Some(virtual_address)
    }
}

impl Protected for ProtectedMemory {
    fn handle_mode<
        T,
        F: FnOnce(&mut ProtectedMemory) -> T,
        G: FnOnce(&mut ProtectedMemory) -> T,
    >(
        &mut self,
        kernel_action: F,
        user_action: G,
    ) -> T {
        match self.current_mode {
            Mode::Kernel => kernel_action(self),
            Mode::User => user_action(self),
        }
    }
}

impl ProtectedMemory {
    pub fn set_mode(&mut self, mode: Mode) {
        self.current_mode = mode;
    }
}

pub struct FixedSizePagedMemory {
    protected_memory: ProtectedMemory,
    current_page: u32,
    page_bits: u32,
    n_pages: u32,
    page_mask: u32,
}

impl FixedSizePagedMemory {
    pub fn new(mem_size: usize, page_bits: u32) -> Self {
        let n_pages = 1 << page_bits;
        let page_mask = n_pages - 1;
        Self {
            protected_memory: ProtectedMemory {
                flat_memory: FlatMemory {
                    memory: vec![0; mem_size as usize],
                },
                current_mode: Mode::Kernel,
            },
            current_page: 0,
            page_bits,
            n_pages,
            page_mask,
        }
    }

    pub fn set_page(&mut self, physical_page: u32) -> bool {
        if physical_page < self.n_pages {
            self.current_page = physical_page;
            true
        } else {
            false
        }
    }

    pub fn translate(&self, virtual_address: Address) -> Option<Address> {
        let physical_address =
            (self.current_page << self.page_bits) | (virtual_address & self.page_mask);
        Some(physical_address as Address)
    }
}

impl Memory for FixedSizePagedMemory {
    fn read(&self, addr: Address) -> Option<Word> {
        let physical_addr = self.translate(addr)?;
        if self.is_valid(physical_addr) {
            self.protected_memory.flat_memory.read(physical_addr)
        } else {
            None
        }
    }

    fn write(&mut self, addr: Address, value: Word) -> bool {
        let physical_address = self.translate(addr);
        let valid =
            physical_address.map_or(false, |physical_address| self.is_valid(physical_address));

        let kernel_action = |mem: &mut ProtectedMemory| mem.write(addr, value);
        let user_action =
            |mem: &mut ProtectedMemory| valid && mem.write(physical_address.unwrap(), value);

        return self
            .protected_memory
            .handle_mode(kernel_action, user_action);
    }

    fn is_valid(&self, addr: Address) -> bool {
        self.protected_memory.is_valid(addr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write() {
        let mut mem = FlatMemory {
            memory: vec![0; 1024],
        };

        // Test write
        assert_eq!(mem.write(0, 42), true);

        // Test read
        assert_eq!(mem.read(0), Some(42));

        // Test invalid address
        assert_eq!(mem.read(2000), None);
    }

    #[test]
    fn test_handle_mode() {
        let mut protected_mem = ProtectedMemory {
            flat_memory: FlatMemory {
                memory: vec![0; 1024],
            },
            current_mode: Mode::Kernel,
        };

        let kernel_result = protected_mem.handle_mode(|_| "Kernel", |_| "User");
        assert_eq!(kernel_result, "Kernel");

        protected_mem.set_mode(Mode::User);

        let user_result = protected_mem.handle_mode(|_| "Kernel", |_| "User");
        assert_eq!(user_result, "User");
    }

    #[test]
    fn test_read_write_protected_memory() {
        let mut protected_mem = ProtectedMemory {
            flat_memory: FlatMemory {
                memory: vec![0; 1024],
            },
            current_mode: Mode::Kernel,
        };

        // Test write in kernel mode
        assert_eq!(protected_mem.write(0, 42), true);

        // Test read in kernel mode
        assert_eq!(protected_mem.read(0), Some(42));

        // Test write in user mode
        protected_mem.set_mode(Mode::User);
        assert_eq!(protected_mem.write(0, 43), true);

        // Test read in user mode
        assert_eq!(protected_mem.read(0), Some(43));
    }

    #[test]
    fn test_fixed_size_paged_memory() {
        let mut paged_mem = FixedSizePagedMemory::new(1024, 4);

        // Test write and read in kernel mode
        assert_eq!(paged_mem.write(16, 42), true);
        assert_eq!(paged_mem.read(16), Some(42));

        // Test write and read in user mode with valid address
        paged_mem.set_page(1);
        assert_eq!(paged_mem.read(0), Some(42));

        // Test write and read in user mode with invalid address
        // assert_eq!(paged_mem.write(2000, 44), false);
        // assert_eq!(paged_mem.read(2000), None);
    }
}

fn main() {}
