#!/bin/bash

declare -A folder_map=(
	["Chapter 0."]="ch00_introduction"
	["Chapter 1. Getting Started"]="ch01_getting_started"
	["Chapter 2. Programming a Guessing Game"]="ch02_programming_a_guessing_game"
	["Chapter 3. Common Programming Concepts"]="ch03_common_programming_concepts"
	["Chapter 4. Understanding Ownership"]="ch04_understanding_ownership"
	["Chapter 5. Using Structs to Structure Related Data"]="ch05_using_structs_to_structure_related_data"
	["Chapter 6. Enums and Pattern Matching"]="ch06_enums_and_pattern_matching"
	["Chapter 7. Managing Growing Projects with Packages, Crates, and Modules"]="ch07_managing_growing_projects_with_packages_crates_and_modules"
	["Chapter 8. Common Collections"]="ch08_common_collections"
	["Chapter 9. Error Handling"]="ch09_error_handling"
	["Chapter 10. Generic Types, Traits, and Lifetimes"]="ch10_generic_types_traits_and_lifetimes"
	["Chapter 11. Writing Automated Tests"]="ch11_writing_automated_tests"
	["Chapter 12. An IO Project Building a Command Line Program"]="ch12_an_io_project_building_a_command_line_program"
	["Chapter 13. Functional Language Features Iterators and Closures"]="ch13_functional_language_features_iterators_and_closures"
	["Chapter 14. More about Cargo and Crates.io"]="ch14_more_about_cargo_and_crates_io"
	["Chapter 15. Smart Pointers"]="ch15_smart_pointers"
	["Chapter 16. Fearless Concurrency"]="ch16_fearless_concurrency"
	["Chapter 17. Object-Oriented Programming Features of Rust"]="ch17_object_oriented_programming_features_of_rust"
	["Chapter 18. Patterns and Matching"]="ch18_patterns_and_matching"
)

for old_name in "${!folder_map[@]}"; do
	new_name="${folder_map[$old_name]}"
	if [ -d "$old_name" ]; then
		git mv "$old_name" "$new_name"
	else
		echo "Folder not found: $old_name"
	fi
done
