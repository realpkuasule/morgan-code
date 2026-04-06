// 关公战秦琼推演分析程序
// 这是一个有趣的假设性对决分析

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Warrior {
    name: String,
    dynasty: String,
    birth_year: i32,
    death_year: i32,
    weapons: Vec<String>,
    skills: Vec<String>,
    attributes: HashMap<String, f64>,
}

impl Warrior {
    fn new(
        name: &str,
        dynasty: &str,
        birth_year: i32,
        death_year: i32,
        weapons: Vec<&str>,
        skills: Vec<&str>,
    ) -> Self {
        let mut attributes = HashMap::new();
        
        // 根据历史记载设置属性
        match name {
            "关羽" => {
                attributes.insert("力量".to_string(), 9.5);
                attributes.insert("敏捷".to_string(), 8.0);
                attributes.insert("智力".to_string(), 7.0);
                attributes.insert("统帅".to_string(), 8.5);
                attributes.insert("武器熟练度".to_string(), 9.8);
                attributes.insert("马术".to_string(), 9.0);
                attributes.insert("忠诚".to_string(), 10.0);
                attributes.insert("声望".to_string(), 9.5);
            }
            "秦琼" => {
                attributes.insert("力量".to_string(), 9.0);
                attributes.insert("敏捷".to_string(), 9.0);
                attributes.insert("智力".to_string(), 7.5);
                attributes.insert("统帅".to_string(), 8.0);
                attributes.insert("武器熟练度".to_string(), 9.5);
                attributes.insert("马术".to_string(), 8.5);
                attributes.insert("忠诚".to_string(), 9.0);
                attributes.insert("声望".to_string(), 8.5);
            }
            _ => {
                attributes.insert("力量".to_string(), 7.0);
                attributes.insert("敏捷".to_string(), 7.0);
                attributes.insert("智力".to_string(), 7.0);
                attributes.insert("统帅".to_string(), 7.0);
                attributes.insert("武器熟练度".to_string(), 7.0);
                attributes.insert("马术".to_string(), 7.0);
                attributes.insert("忠诚".to_string(), 7.0);
                attributes.insert("声望".to_string(), 7.0);
            }
        }
        
        Self {
            name: name.to_string(),
            dynasty: dynasty.to_string(),
            birth_year,
            death_year,
            weapons: weapons.iter().map(|s| s.to_string()).collect(),
            skills: skills.iter().map(|s| s.to_string()).collect(),
            attributes,
        }
    }
    
    fn get_attribute(&self, key: &str) -> f64 {
        *self.attributes.get(key).unwrap_or(&0.0)
    }
    
    fn calculate_combat_power(&self) -> f64 {
        let mut power = 0.0;
        
        power += self.get_attribute("力量") * 1.5;
        power += self.get_attribute("敏捷") * 1.2;
        power += self.get_attribute("武器熟练度") * 1.8;
        power += self.get_attribute("马术") * 1.0;
        power += self.get_attribute("智力") * 0.8;
        power += self.get_attribute("声望") * 0.5;
        
        // 特殊加成
        if self.name == "关羽" {
            power += 2.0; // 青龙偃月刀加成
            if self.weapons.contains(&"赤兔马".to_string()) {
                power += 1.5;
            }
        } else if self.name == "秦琼" {
            power += 1.5; // 双锏加成
        }
        
        power
    }
    
    fn display_info(&self) {
        println!("武将: {}", self.name);
        println!("朝代: {}", self.dynasty);
        println!("生卒: {}年 - {}年", self.birth_year, self.death_year);
        println!("武器: {}", self.weapons.join(", "));
        println!("技能: {}", self.skills.join(", "));
        println!("属性:");
        for (key, value) in &self.attributes {
            println!("  {}: {:.1}", key, value);
        }
        println!("综合战力: {:.2}", self.calculate_combat_power());
        println!();
    }
}

#[derive(Debug)]
struct BattleScenario {
    location: String,
    terrain: String,
    weather: String,
    time_period: String,
    rules: String,
}

impl BattleScenario {
    fn new(location: &str, terrain: &str, weather: &str, time_period: &str, rules: &str) -> Self {
        Self {
            location: location.to_string(),
            terrain: terrain.to_string(),
            weather: weather.to_string(),
            time_period: time_period.to_string(),
            rules: rules.to_string(),
        }
    }
    
    fn get_terrain_modifier(&self, warrior: &Warrior) -> f64 {
        let mut modifier = 1.0;
        
        match self.terrain.as_str() {
            "平原" => {
                if warrior.name == "关羽" && warrior.weapons.contains(&"赤兔马".to_string()) {
                    modifier = 1.2; // 赤兔马在平原上有优势
                }
            }
            "山地" => {
                if warrior.weapons.contains(&"双锏".to_string()) {
                    modifier = 1.1; // 双锏在山地更灵活
                }
            }
            "城池" => modifier = 1.0,
            _ => modifier = 1.0,
        }
        
        match self.weather.as_str() {
            "晴天" => modifier *= 1.0,
            "雨天" => {
                if warrior.weapons.contains(&"青龙偃月刀".to_string()) {
                    modifier *= 0.9; // 重武器在雨天受影响
                }
            }
            "雪天" => modifier *= 0.8,
            _ => modifier *= 1.0,
        }
        
        modifier
    }
}

fn simulate_battle(warrior1: &Warrior, warrior2: &Warrior, scenario: &BattleScenario) -> (f64, f64, String) {
    let mut score1 = warrior1.calculate_combat_power();
    let mut score2 = warrior2.calculate_combat_power();
    
    // 地形天气修正
    score1 *= scenario.get_terrain_modifier(warrior1);
    score2 *= scenario.get_terrain_modifier(warrior2);
    
    // 时代差异修正
    let time_diff = (warrior2.birth_year - warrior1.birth_year).abs();
    if time_diff > 0 {
        // 时代越晚，战术和技术可能越先进
        let time_modifier = 1.0 + (time_diff as f64 * 0.001);
        if warrior2.birth_year > warrior1.birth_year {
            score2 *= time_modifier;
        } else {
            score1 *= time_modifier;
        }
    }
    
    // 分析对决结果
    let score_diff = score1 - score2;
    let mut result = String::new();
    
    if score_diff.abs() < 2.0 {
        result = format!("{} 与 {} 实力相当，战斗将异常激烈，可能持续数百回合不分胜负！",
            warrior1.name, warrior2.name);
    } else if score1 > score2 {
        result = format!("{} 略占优势！{} 可能需要依赖战术和智谋来弥补力量上的差距。",
            warrior1.name, warrior2.name);
        
        if score_diff > 5.0 {
            result = format!("{} 明显占优！{} 将面临巨大压力，需要寻找战机。",
                warrior1.name, warrior2.name);
        }
    } else {
        result = format!("{} 略占优势！{} 可能需要依赖战术和智谋来弥补力量上的差距。",
            warrior2.name, warrior1.name);
        
        if score_diff.abs() > 5.0 {
            result = format!("{} 明显占优！{} 将面临巨大压力，需要寻找战机。",
                warrior2.name, warrior1.name);
        }
    }
    
    (score1, score2, result)
}

fn analyze_matchup(warrior1: &Warrior, warrior2: &Warrior) {
    println!("【对阵分析】");
    println!("{} ({}朝) vs {} ({}朝)", 
        warrior1.name, warrior1.dynasty, 
        warrior2.name, warrior2.dynasty);
    println!("时间跨度: 约{}年", 
        (warrior2.birth_year - warrior1.birth_year).abs());
    println!();
    
    // 优势对比
    println!("【优势对比】");
    let attributes = vec!["力量", "敏捷", "智力", "统帅", "武器熟练度", "马术"];
    
    for attr in attributes {
        let val1 = warrior1.get_attribute(attr);
        let val2 = warrior2.get_attribute(attr);
        
        if val1 > val2 {
            println!("{}: {} ({:.1}) > {} ({:.1})", 
                attr, warrior1.name, val1, warrior2.name, val2);
        } else if val2 > val1 {
            println!("{}: {} ({:.1}) > {} ({:.1})", 
                attr, warrior2.name, val2, warrior1.name, val1);
        } else {
            println!("{}: 双方持平 ({:.1})", attr, val1);
        }
    }
    println!();
}

fn main() {
    println!("==========================================");
    println!("        关公战秦琼 - 跨时代对决推演");
    println!("==========================================");
    println!();
    
    // 创建关羽
    let guan_yu = Warrior::new(
        "关羽",
        "三国",
        160,
        220,
        vec!["青龙偃月刀", "赤兔马"],
        vec!["拖刀计", "春秋刀法", "过五关斩六将", "单刀赴会"],
    );
    
    // 创建秦琼
    let qin_qiong = Warrior::new(
        "秦琼",
        "隋唐",
        571,
        638,
        vec!["双锏", "虎头錾金枪", "黄骠马"],
        vec!["撒手锏", "秦家锏法", "马踏黄河两岸", "锏打三州六府"],
    );
    
    // 显示武将信息
    println!("【武将信息】");
    guan_yu.display_info();
    qin_qiong.display_info();
    
    // 对阵分析
    analyze_matchup(&guan_yu, &qin_qiong);
    
    // 不同场景下的推演
    let scenarios = vec![
        BattleScenario::new("虎牢关", "平原", "晴天", "白天", "马上对决"),
        BattleScenario::new("瓦岗寨", "山地", "雨天", "白天", "步战"),
        BattleScenario::new("长安城外", "平原", "雪天", "清晨", "公平对决"),
        BattleScenario::new("校场", "平坦", "阴天", "正午", "比武切磋"),
    ];
    
    println!("【不同场景推演】");
    for (i, scenario) in scenarios.iter().enumerate() {
        println!("=== 场景 {}: {} ({}, {}) ===", 
            i + 1, scenario.location, scenario.terrain, scenario.weather);
        println!("时间: {}, 规则: {}", scenario.time_period, scenario.rules);
        
        let (score1, score2, result) = simulate_battle(&guan_yu, &qin_qiong, scenario);
        
        println!("{} 修正战力: {:.2}", guan_yu.name, score1);
        println!("{} 修正战力: {:.2}", qin_qiong.name, score2);
        println!("推演结果: {}", result);
        println!();
    }
    
    // 综合评估
    println!("【综合评估】");
    let avg_score1: f64 = scenarios.iter()
        .map(|s| simulate_battle(&guan_yu, &qin_qiong, s).0)
        .sum::<f64>() / scenarios.len() as f64;
    
    let avg_score2: f64 = scenarios.iter()
        .map(|s| simulate_battle(&guan_yu, &qin_qiong, s).1)
        .sum::<f64>() / scenarios.len() as f64;
    
    println!("平均战力 - {}: {:.2}", guan_yu.name, avg_score1);
    println!("平均战力 - {}: {:.2}", qin_qiong.name, avg_score2);
    
    if avg_score1 > avg_score2 {
        println!("\n🌟 综合来看，{} 略胜一筹！", guan_yu.name);
        println!("原因分析:");
        println!("1. 青龙偃月刀的威力和赤兔马的速度组合");
        println!("2. 三国时期更频繁的实战经验");
        println!("3. 关羽的忠诚和声望带来的士气加成");
    } else if avg_score2 > avg_score1 {
        println!("\n🌟 综合来看，{} 略胜一筹！", qin_qiong.name);
        println!("原因分析:");
        println!("1. 双锏的灵活性和虎头錾金枪的攻击范围");
        println!("2. 隋唐时期更先进的武器和战术");
        println!("3. 秦琼的智谋和统帅能力");
    } else {
        println!("\n🌟 双方实力相当，胜负难料！");
        println!("这真正是一场势均力敌的巅峰对决！");
    }
    
    println!();
    println!("【历史点评】");
    println!("关羽和秦琼都是各自时代的顶尖武将，");
    println!("关羽以忠义和武力著称，秦琼以勇猛和智谋闻名。");
    println!("这场跨越400年的对决虽然不可能发生，");
    println!("但通过推演我们可以看到两位英雄各自的优势。");
    println!("无论胜负如何，他们都是中华民族的骄傲！");
    println!();
    println!("==========================================");
}