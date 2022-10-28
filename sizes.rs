fn bound<T>(n: usize) -> usize { 
    (n + 255) /256 + (n + 32) * std::mem::size_of::<T>()
}

fn bound_p4<T>(n: usize) -> usize {
    (n+127)/128+(n+32)*std::mem::size_of::<T>()
}
