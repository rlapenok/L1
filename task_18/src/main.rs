fn main() {
    let data="Бить будете, папаша? 和平 γ 🌟".chars().rev().collect::<String>();
    assert_eq!("🌟 γ 平和 ?ашапап ,етедуб ьтиБ",data);
}
