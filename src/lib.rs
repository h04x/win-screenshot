// https://stackoverflow.com/questions/7292757/how-to-get-screenshot-of-a-window-as-bitmap-object-in-c
// https://stackoverflow.com/questions/37132196/multi-monitor-screenshots-only-2-monitors-in-c-with-winapi
// https://superkogito.github.io/blog/2020/09/28/loop_monitors_details_in_cplusplus.html
// https://stackoverflow.com/questions/63826570/printwindow-on-rtl-window-results-in-a-mirrored-image-with-pw-renderfullcontent
// https://stackoverflow.com/questions/36261725/how-to-extract-a-part-of-hbitmap-without-using-bitblt
// https://stackoverflow.com/questions/3671008/crop-function-bitblt

pub mod capture;
pub mod prelude;
#[cfg(test)]
mod tests;
pub mod utils;
mod wrappers;
