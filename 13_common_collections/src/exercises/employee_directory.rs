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

#[cfg(test)]
mod unit_tests_exercises_3 {

    use super::*;

    #[test]
    fn add_employee_stores_employee_in_department() {
        let mut company = EmployeeDirectory::new();

        let emp1 = Employee {
            username: String::from("alice"),
            email: String::from("alice@company.com"),
        };

        company.add_employee(Department::Backend, emp1);

        let backend_employees = company.get_employees_by_department(&Department::Backend);

        assert!(backend_employees.is_some());

        let employees = backend_employees.unwrap();
        assert_eq!(employees.len(), 1);
        assert_eq!(employees[0].username, "alice");
        assert_eq!(employees[0].email, "alice@company.com");
    }

    #[test]
    fn show_directory_does_not_panic() {
        let mut company = EmployeeDirectory::new();

        let emp1 = Employee {
            username: String::from("alice"),
            email: String::from("alice@company.com"),
        };

        let emp2 = Employee {
            username: String::from("bob"),
            email: String::from("bob@company.com"),
        };

        let emp3 = Employee {
            username: String::from("charlie"),
            email: String::from("charlie@company.com"),
        };

        company.add_employee(Department::Backend, emp1);
        company.add_employee(Department::Backend, emp2);
        company.add_employee(Department::Backend, emp3);

        company.show_directory();
    }

    #[test]
    fn get_employees_by_department_returns_correct_employees() {
        let mut company = EmployeeDirectory::new();

        let emp1 = Employee {
            username: String::from("alice"),
            email: String::from("alice@company.com"),
        };

        let emp2 = Employee {
            username: String::from("bob"),
            email: String::from("bob@company.com"),
        };

        let emp3 = Employee {
            username: String::from("charlie"),
            email: String::from("charlie@company.com"),
        };

        company.add_employee(Department::Backend, emp1);
        company.add_employee(Department::Backend, emp2);
        company.add_employee(Department::Frontend, emp3);

        let backend_employees = company.get_employees_by_department(&Department::Backend);
        let frontend_employees = company.get_employees_by_department(&Department::Frontend);
        let fullstack_employees = company.get_employees_by_department(&Department::Fullstack);
        assert!(fullstack_employees.is_none());

        assert!(backend_employees.is_some());
        assert!(frontend_employees.is_some());

        let backend_employees = backend_employees.unwrap();
        assert_eq!(backend_employees.len(), 2);

        let frontend_employees = frontend_employees.unwrap();
        assert_eq!(frontend_employees.len(), 1);
        assert_eq!(frontend_employees[0].username, "charlie");
        assert_eq!(frontend_employees[0].email, "charlie@company.com");
    }
}
