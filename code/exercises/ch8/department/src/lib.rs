use std::collections::HashMap;

pub struct Company {
    data: HashMap<String, Vec<String>>,
}

impl Default for Company {
    fn default() -> Self {
        Self::new()
    }
}

impl Company {
    pub fn new() -> Company {
        Company {
            data: HashMap::new(),
        }
    }

    pub fn add_employee(&mut self, employee: &str, department: &str) {
        self.data
            .entry(department.to_string())
            .or_insert_with(Vec::new)
            .push(employee.to_string());
    }

    pub fn remove_employee(&mut self, employee: &str, department: &str) -> Result<(), String> {
        if let Some(employees) = self.data.get_mut(department) {
            if let Some(index) = employees.iter().position(|e| e == employee) {
                employees.remove(index);
                return Ok(());
            } else {
                return Err(format!(
                    "Employee {} not found in the {} department.",
                    employee, department
                ));
            }
        }
        Err(format!("Department {} not found.", department))
    }

    pub fn list_department(&self, department: &str) -> Option<Vec<String>> {
        self.data.get(department).map(|employees| {
            let mut sorted = employees.clone();
            sorted.sort();
            sorted
        })
    }

    pub fn list_all(&self) -> Vec<(String, Vec<String>)> {
        let mut all_departments: Vec<(String, Vec<String>)> = self
            .data
            .iter()
            .map(|(department, employees)| {
                let mut sorted = employees.clone();
                sorted.sort();
                (department.clone(), sorted)
            })
            .collect();
        all_departments.sort_by_key(|(department, _)| department.clone());
        all_departments
    }

    pub fn process_input(&mut self, input: &str) -> String {
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() >= 4 && parts[0] == "Add" && parts[2] == "to" {
            let employee = parts[1];
            let department = parts[3];
            self.add_employee(employee, department);
            format!("Added {} to {} department.", employee, department)
        } else if parts.len() >= 4 && parts[0] == "Remove" && parts[2] == "from" {
            let employee = parts[1];
            let department = parts[3];
            match self.remove_employee(employee, department) {
                Ok(()) => format!("Removed {} from the {} department.", employee, department),
                Err(message) => message,
            }
        } else if parts.len() == 2 && parts[0] == "List" {
            let department = parts[1];
            match self.list_department(department) {
                Some(employees) => {
                    format!(
                        "Employees in the {} department:\n{}",
                        department,
                        employees.join("\n")
                    )
                }
                None => format!("No employees found in the {} department.", department),
            }
        } else if input == "List all" {
            let all_departments = self.list_all();
            all_departments
                .iter()
                .map(|(department, employees)| {
                    format!(
                        "Employees in the {} department:\n{}",
                        department,
                        employees.join("\n")
                    )
                })
                .collect::<Vec<String>>()
                .join("\n\n")
        } else {
            String::from("Invalid command. Please try again.")
        }
    }
}
