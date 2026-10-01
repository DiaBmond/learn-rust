use std::collections::HashMap;

#[derive(Debug)]
pub struct Employee {
    pub username: String,
    pub email: String,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Department {
    Frontend,
    Backend,
    Fullstack,
    Mobile,
    DevOps,
    Tester,
}

pub struct EmployeeDirectory {
    pub directory: HashMap<Department, Vec<Employee>>,
}

impl EmployeeDirectory {
    pub fn new() -> Self {
        Self {
            directory: HashMap::new(),
        }
    }

    pub fn add_employee(&mut self, department: Department, employee: Employee) {
        self.directory
            .entry(department)
            .or_insert_with(Vec::new)
            .push(employee);
    }

    pub fn show_directory(&self) {
        for (department, employees) in &self.directory {
            println!("=== {:?} ({}) ===", department, employees.len());
            for employee in employees {
                println!("- {} <{}>", employee.username, employee.email);
            }
        }
    }

    pub fn get_employees_by_department(&self, department: &Department) -> Option<&Vec<Employee>> {
        self.directory.get(department)
    }
}
