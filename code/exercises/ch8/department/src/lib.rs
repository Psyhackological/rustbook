use std::collections::HashMap;

// Define a Company struct that will store department data
pub struct Company {
    data: HashMap<String, Vec<String>>,
}

// Implement the Default trait for Company, allowing us to create an instance with default values
impl Default for Company {
    fn default() -> Self {
        Self::new()
    }
}

impl Company {
    // Constructor for the Company struct
    pub fn new() -> Self {
        Self {
            // Initialize an empty HashMap to store departments and their employees
            data: HashMap::new(),
        }
    }

    // Add an employee to a department
    pub fn add_employee(&mut self, employee: &str, department: &str) {
        self.data
            .entry(department.to_string())
            // If the department exists, use its employee list; otherwise, create a new Vec
            .or_insert_with(Vec::new)
            // Add the employee to the department's list
            .push(employee.to_string());
    }

    // Remove an employee from a department
    pub fn remove_employee(&mut self, employee: &str, department: &str) -> Result<(), String> {
        // Check if the department exists in the HashMap
        if let Some(employees) = self.data.get_mut(department) {
            // Find the index of the employee in the department's list
            if let Some(index) = employees.iter().position(|e| e == employee) {
                // Remove the employee from the list
                employees.remove(index);
                // Remove the department entry if there are no employees left in the department
                if employees.is_empty() {
                    self.data.remove(department);
                }
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

    pub fn move_employee(
        &mut self,
        employee: &str,
        from_department: &str,
        to_department: &str,
    ) -> Result<(), String> {
        // Remove the employee from the old department
        match self.remove_employee(employee, from_department) {
            Ok(()) => {
                // Add the employee to the new department
                self.add_employee(employee, to_department);
                Ok(())
            }
            Err(message) => Err(message),
        }
    }

    // List all employees in a department, sorted alphabetically
    pub fn list_department(&self, department: &str) -> Option<Vec<String>> {
        self.data.get(department).and_then(|employees| {
            let mut sorted = employees.clone();
            sorted.sort();
            // Check if the department has no employees
            if sorted.is_empty() {
                None
            } else {
                Some(sorted)
            }
        }) // Flatten the result to get rid of nested Option
    }

    // List all employees in the company by department, sorted alphabetically
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

    // Process user input and generate output based on the command
    pub fn process_input(&mut self, input: &str) -> String {
        let parts: Vec<&str> = input.split_whitespace().collect();

        // Add employee to a department
        if parts.len() >= 4 && parts[0] == "Add" && parts[2] == "to" {
            let employee = parts[1];
            let department = parts[3];
            self.add_employee(employee, department);
            format!("Added {} to {} department.", employee, department)
        }
        // Remove employee from a department
        else if parts.len() >= 4 && parts[0] == "Remove" && parts[2] == "from" {
            let employee = parts[1];
            let department = parts[3];
            match self.remove_employee(employee, department) {
                Ok(()) => format!("Removed {} from the {} department.", employee, department),
                Err(message) => message,
            }
        }
        // List all employees in the company by department
        else if input.trim() == "List All" {
            let all_departments = self.list_all();
            if all_departments.is_empty() {
                String::from("No employees found in the company.")
            } else {
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
            }
        }
        // List employees in a specific department
        else if parts.len() == 2 && parts[0] == "List" {
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
        }
        // Move employee to another department
        else if parts.len() >= 6 && parts[0] == "Move" && parts[2] == "from" && parts[4] == "to" {
            let employee = parts[1];
            let from_department = parts[3];
            let to_department = parts[5];
            match self.move_employee(employee, from_department, to_department) {
                Ok(()) => format!(
                    "Moved {} from the {} department to the {} department.",
                    employee, from_department, to_department
                ),
                Err(message) => message,
            }
        } else {
            // Handle invalid commands
            String::from("Invalid command. Please try again.")
        }
    }
}
