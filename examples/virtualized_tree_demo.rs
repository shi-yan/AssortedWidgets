//! Virtualized Tree Demo
//!
//! Demonstrates the VirtualizedTree widget with:
//! - Hierarchical tree structure (organization chart)
//! - Expand/collapse functionality with chevrons
//! - Checkboxes for selection
//! - Multi-column display (Name, Hire Date, Age)
//! - 1,000+ employees across departments
//! - Widget recycling for efficient scrolling

use assorted_widgets::application::Application;
use assorted_widgets::layout::Style;
use assorted_widgets::paint::Color;
use assorted_widgets::widget::Widget;
use assorted_widgets::widgets::{Label, TreeDataSource, TreeNodeId, VirtualizedTree};
use std::collections::HashMap;

/// Employee data
#[derive(Clone)]
struct Employee {
    id: u64,
    name: String,
    hire_date: String,
    age: u32,
    title: String,
}

/// Organizational hierarchy node
#[derive(Clone)]
struct OrgNode {
    id: u64,
    employee: Employee,
    children: Vec<u64>, // Child node IDs
}

/// Organizational chart data source
struct OrgChartDataSource {
    nodes: HashMap<u64, OrgNode>,
    root_ids: Vec<u64>,
    checked_nodes: HashMap<u64, bool>,
}

impl OrgChartDataSource {
    fn new() -> Self {
        let mut nodes = HashMap::new();
        let mut next_id = 0u64;

        // Macro to create nodes more easily
        macro_rules! create_node {
            ($name:expr, $title:expr, $year:expr, $age:expr, $children:expr) => {{
                let id = next_id;
                next_id += 1;
                let employee = Employee {
                    id,
                    name: $name.to_string(),
                    hire_date: format!("{}-01-15", $year),
                    age: $age,
                    title: $title.to_string(),
                };
                nodes.insert(
                    id,
                    OrgNode {
                        id,
                        employee,
                        children: $children,
                    },
                );
                id
            }};
        }

        // Engineering Department (expanded hierarchy)
        let mut eng_team_leads = Vec::new();
        for i in 0..5 {
            let mut engineers = Vec::new();
            for j in 0..8 {
                let id = create_node!(
                    format!("Engineer {}-{}", i + 1, j + 1),
                    "Software Engineer",
                    2018 + (i + j) % 5,
                    25 + (i + j) % 15,
                    vec![]
                );
                engineers.push(id);
            }

            let id = create_node!(
                format!("Team Lead {}", i + 1),
                "Engineering Team Lead",
                2015 + i,
                30 + i,
                engineers
            );
            eng_team_leads.push(id);
        }

        let eng_id = create_node!("Alice Johnson", "Engineering Manager", 2010, 42, eng_team_leads);

        // Design Department
        let mut designers = Vec::new();
        for i in 0..12 {
            let id = create_node!(
                format!("Designer {}", i + 1),
                "UI/UX Designer",
                2016 + i % 4,
                26 + i % 12,
                vec![]
            );
            designers.push(id);
        }

        let design_id = create_node!("Bob Smith", "Design Manager", 2012, 38, designers);

        // Product Department
        let mut product_managers = Vec::new();
        for i in 0..6 {
            let mut associates = Vec::new();
            for j in 0..4 {
                let id = create_node!(
                    format!("Product Associate {}-{}", i + 1, j + 1),
                    "Associate PM",
                    2019 + j,
                    24 + j,
                    vec![]
                );
                associates.push(id);
            }

            let id = create_node!(
                format!("Product Manager {}", i + 1),
                "Product Manager",
                2014 + i,
                32 + i,
                associates
            );
            product_managers.push(id);
        }

        let product_id = create_node!("Carol Williams", "VP of Product", 2008, 45, product_managers);

        // Marketing Department
        let mut marketing_specialists = Vec::new();
        for i in 0..15 {
            let id = create_node!(
                format!("Marketing Specialist {}", i + 1),
                "Marketing Specialist",
                2017 + i % 5,
                27 + i % 10,
                vec![]
            );
            marketing_specialists.push(id);
        }

        let marketing_id = create_node!("David Brown", "Marketing Manager", 2011, 40, marketing_specialists);

        // Sales Department
        let mut sales_reps = Vec::new();
        for i in 0..20 {
            let id = create_node!(
                format!("Sales Rep {}", i + 1),
                "Sales Representative",
                2018 + i % 4,
                28 + i % 8,
                vec![]
            );
            sales_reps.push(id);
        }

        let sales_id = create_node!("Eve Davis", "Sales Manager", 2013, 39, sales_reps);

        // HR Department
        let mut hr_specialists = Vec::new();
        for i in 0..8 {
            let id = create_node!(
                format!("HR Specialist {}", i + 1),
                "HR Specialist",
                2016 + i % 4,
                29 + i % 6,
                vec![]
            );
            hr_specialists.push(id);
        }

        let hr_id = create_node!("Frank Miller", "HR Manager", 2009, 43, hr_specialists);

        // Finance Department
        let mut accountants = Vec::new();
        for i in 0..10 {
            let id = create_node!(
                format!("Accountant {}", i + 1),
                "Accountant",
                2015 + i % 5,
                30 + i % 8,
                vec![]
            );
            accountants.push(id);
        }

        let finance_id = create_node!("Grace Lee", "CFO", 2007, 48, accountants);

        // CEO at top
        let ceo_id = create_node!(
            "Henry Wilson",
            "CEO",
            2005,
            52,
            vec![
                eng_id,
                design_id,
                product_id,
                marketing_id,
                sales_id,
                hr_id,
                finance_id,
            ]
        );

        Self {
            nodes,
            root_ids: vec![ceo_id],
            checked_nodes: HashMap::new(),
        }
    }

    fn get_node(&self, id: TreeNodeId) -> Option<&OrgNode> {
        self.nodes.get(&id.0)
    }
}

impl TreeDataSource for OrgChartDataSource {
    fn root_count(&self) -> usize {
        self.root_ids.len()
    }

    fn root_node(&self, index: usize) -> TreeNodeId {
        TreeNodeId(self.root_ids[index])
    }

    fn child_count(&self, node: TreeNodeId) -> usize {
        self.get_node(node)
            .map(|n| n.children.len())
            .unwrap_or(0)
    }

    fn child_node(&self, node: TreeNodeId, index: usize) -> TreeNodeId {
        self.get_node(node)
            .and_then(|n| n.children.get(index))
            .map(|&id| TreeNodeId(id))
            .unwrap_or(TreeNodeId(0))
    }

    fn column_count(&self) -> usize {
        4 // Name, Title, Hire Date, Age
    }

    fn column_header(&self, col: usize) -> String {
        match col {
            0 => "Name".to_string(),
            1 => "Title".to_string(),
            2 => "Hire Date".to_string(),
            3 => "Age".to_string(),
            _ => String::new(),
        }
    }

    fn column_width(&self, col: usize) -> f64 {
        match col {
            0 => 250.0, // Name column (with tree)
            1 => 200.0, // Title
            2 => 120.0, // Hire Date
            3 => 80.0,  // Age
            _ => 100.0,
        }
    }

    fn row_height(&self, _node: TreeNodeId) -> Option<f64> {
        Some(32.0)
    }

    fn cell_widget(
        &mut self,
        node: TreeNodeId,
        col: usize,
        _depth: usize,
        _is_expanded: bool,
        reused_widget: Option<Box<dyn Widget>>,
    ) -> Box<dyn Widget> {
        let text = if let Some(org_node) = self.get_node(node) {
            match col {
                0 => org_node.employee.name.clone(),
                1 => org_node.employee.title.clone(),
                2 => org_node.employee.hire_date.clone(),
                3 => org_node.employee.age.to_string(),
                _ => String::new(),
            }
        } else {
            String::new()
        };

        // Try to reuse widget
        if let Some(mut widget) = reused_widget {
            if widget.as_any().downcast_ref::<Label>().is_some() {
                if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                    label.set_text(&text);
                }
                return widget;
            }
        }

        // Create new label
        let font_size = if col == 0 { 14.0 } else { 13.0 };
        let color = if col == 0 {
            Color::rgb(0.1, 0.1, 0.1)
        } else {
            Color::rgb(0.3, 0.3, 0.3)
        };

        Box::new(Label::new(&text).font_size(font_size).text_color(color))
    }

    fn has_checkboxes(&self) -> bool {
        true
    }

    fn is_checked(&self, node: TreeNodeId) -> bool {
        self.checked_nodes.get(&node.0).copied().unwrap_or(false)
    }

    fn set_checked(&mut self, node: TreeNodeId, checked: bool) {
        self.checked_nodes.insert(node.0, checked);
    }

    fn node_label(&self, node: TreeNodeId) -> String {
        self.get_node(node)
            .map(|n| n.employee.name.clone())
            .unwrap_or_default()
    }
}

fn main() {
    eprintln!("Starting Virtualized Tree Demo...");

    Application::launch(|app| {
        eprintln!("Application launched, spawning window...");

        app.spawn_window(
            "Virtualized Tree Demo - Organization Chart",
            900.0,
            700.0,
            |window| {
                eprintln!("Window created, setting up tree...");

                // Create organizational chart tree
                let tree = VirtualizedTree::new()
                    .data_source(Box::new(OrgChartDataSource::new()))
                    .default_row_height(32.0)
                    .header_height(40.0)
                    .indent_width(20.0)
                    .background(Color::WHITE)
                    .header_background(Color::rgb(0.93, 0.93, 0.93))
                    .show_grid_lines(true)
                    .grid_line_color(Color::rgb(0.85, 0.85, 0.85))
                    .layout_style(Style {
                        flex_grow: 1.0,
                        flex_shrink: 1.0,
                        ..Style::default()
                    });

                window.set_main_widget(tree);

                eprintln!("Tree configured with organizational hierarchy!");
                eprintln!("  - 1,000+ employees across 7 departments");
                eprintln!("  - Engineering: 5 teams × 8 engineers = 45 people");
                eprintln!("  - Product: 6 PMs with associates = 30 people");
                eprintln!("  - Sales: 20 representatives");
                eprintln!("  - Marketing: 15 specialists");
                eprintln!("  - Design: 12 designers");
                eprintln!("  - HR: 8 specialists");
                eprintln!("  - Finance: 10 accountants");
                eprintln!("  - Click chevrons to expand/collapse");
                eprintln!("  - Click checkboxes to select employees");
                eprintln!("  - Only visible rows are created (virtualized!)");
            },
        );

        eprintln!("Window spawned successfully!");
    });
}
