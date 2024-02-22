


fn subsets(nums: Vec<i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    backtrack(&nums, 0, &mut current, &mut result);
    result
}

fn backtrack(nums: &Vec<i32>, start: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    // Adiciona a combinação atual ao resultado; clonamos `current` porque seu conteúdo muda ao longo do tempo
    result.push(current.clone());
    
    // Explora mais possibilidades adicionando elementos subsequentes
    for i in start..nums.len() {
        // Adiciona o elemento atual e explora
        current.push(nums[i]);
        backtrack(nums, i + 1, current, result);
        // Remove o último elemento adicionado quando volta para tentar outra possibilidade
        current.pop();
    }
}



fn main() {
    let nums = vec![1, 2, 3];
    let result = subsets(nums);
    for subset in result {
        println!("{:?}", subset);
    }   
}
