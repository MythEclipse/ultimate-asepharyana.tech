pub fn is_internet_baik_block_page(data: &str) -> bool {
    data.contains("internetbaik.telkomsel.com") ||
    data.contains("VmaxAdManager.js") ||
    data.contains("VmaxAdHelper")
}
