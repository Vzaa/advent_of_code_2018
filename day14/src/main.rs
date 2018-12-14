fn main() {
    // Part 1
    {
        let input = 633601;
        let mut elf_1 = 0;
        let mut elf_2 = 1;
        let mut recipes = vec![3, 7];
        loop {
            let mut sum = recipes[elf_1] + recipes[elf_2];

            let mut new_recipes = vec![];
            loop {
                new_recipes.push(sum % 10);
                sum /= 10;
                if sum == 0 {
                    break;
                }
            }

            // Reverse the order
            for n in new_recipes.into_iter().rev() {
                recipes.push(n);
            }

            elf_1 = (elf_1 + recipes[elf_1] + 1) % recipes.len();
            elf_2 = (elf_2 + recipes[elf_2] + 1) % recipes.len();

            if recipes.len() >= input + 10 {
                break;
            }
        }

        let out = &recipes[input..input + 10];
        println!("Part 1: {:?}", out);
    }

    // Part 2
    {
        let input = [6, 3, 3, 6, 0, 1];
        let mut elf_1 = 0;
        let mut elf_2 = 1;
        let mut recipes = vec![3_u8, 7];

        loop {
            let mut sum = recipes[elf_1] + recipes[elf_2];

            let mut new_recipes = vec![];
            loop {
                new_recipes.push(sum % 10);
                sum /= 10;
                if sum == 0 {
                    break;
                }
            }

            // Reverse the order
            for n in new_recipes.into_iter().rev() {
                recipes.push(n);
            }

            elf_1 = (elf_1 + (recipes[elf_1] as usize) + 1) % recipes.len();
            elf_2 = (elf_2 + (recipes[elf_2] as usize) + 1) % recipes.len();

            if recipes.ends_with(&input) {
                println!("Part 2: {}", recipes.len() - input.len());
                break;
            }

            // Pop last one and try again
            let tmp = recipes.pop().unwrap();

            if recipes.ends_with(&input) {
                println!("Part 2: {}", recipes.len() - input.len());
                break;
            }
            recipes.push(tmp);
        }
    }
}
