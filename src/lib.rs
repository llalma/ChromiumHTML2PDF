use pyo3::{prelude::*, wrap_pyfunction};

use futures::StreamExt;

use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::page::PrintToPdfParams;

#[pyfunction]
fn convert<'p>(
    py: Python<'p>, 
    input_path: &'p PyAny,
    output_path: &'p PyAny,
    chromeium_path: Option<&'p PyAny>  ) -> PyResult<&'p PyAny> {
    
    // Get the input parameters
    let input_path: String = input_path.extract().unwrap();
    let output_path: String = output_path.extract().unwrap();

    // Create the BrowserConfig
    let config: BrowserConfig = match chromeium_path{
        Some(v) => {
            let path: String = v.extract().unwrap();
            BrowserConfig::with_executable(path)
        },
        _ => BrowserConfig::builder().build().unwrap()
    };

    pyo3_asyncio::tokio::future_into_py_with_locals(
        py,
        pyo3_asyncio::tokio::get_current_locals(py)?,
        async move {
            
            // Create the browser instance
            let (browser, mut handler) = Browser::launch(config).await.unwrap();
            
            // Create the thread handle
            let _handle = tokio::task::spawn(async move {
                while let Some(h) = handler.next().await {
                    if h.is_err() {
                        break;
                    }
                }
            });

            // Open the input file
            let page = browser
                .new_page(input_path)
                .await
                .unwrap();
            page.wait_for_navigation().await.unwrap();
            
            // Save the html to pdf
            page.save_pdf(PrintToPdfParams::default(), &output_path).await.unwrap();

            // Close the page
            page.close().await.unwrap();

            // Exit with a succesful
            Python::with_gil(|py| Ok(py.None()))
        }
    )
}
    


/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn html2pdf(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert, m)?)?;
    Ok(())
}