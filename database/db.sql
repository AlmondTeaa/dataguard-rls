CREATE TABLE employees (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    department TEXT NOT NULL,
    google_id TEXT NOT NULL
);

INSERT INTO employees (name, department, google_id) VALUES
('Alice', 'IT', '108022246894557453386'),
('Bob', 'IT', '108022246894557453386'),
('Charlie', 'Accounting', '107599949044387531471'),
('Dave', 'Accounting', '107599949044387531471');

ALTER TABLE employees ENABLE ROW LEVEL SECURITY;

CREATE POLICY employee_department_policy
ON employees
USING (google_id = current_setting('app.user_id'));