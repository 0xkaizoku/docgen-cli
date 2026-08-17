use crate::engine::html::render_html_string;
use crate::templates::DocumentMeta;
use anyhow::{anyhow, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

struct TempFileGuard {
    paths: Vec<PathBuf>,
}

impl Drop for TempFileGuard {
    fn drop(&mut self) {
        for path in &self.paths {
            let _ = fs::remove_file(path);
        }
    }
}

pub fn render_pdf_bytes(markdown_input: &str, meta: &DocumentMeta) -> Result<Vec<u8>> {
    let html_content = render_html_string(markdown_input, meta)?;

    let temp_dir = std::env::temp_dir();
    let temp_id = uuid_like_nonce();
    let temp_html_path = temp_dir.join(format!("cli_doc_{}.html", temp_id));
    let temp_pdf_path = temp_dir.join(format!("cli_doc_{}.pdf", temp_id));

    fs::write(&temp_html_path, &html_content)?;

    // Guard will automatically clean up files on return/error
    let _guard = TempFileGuard {
        paths: vec![temp_html_path.clone(), temp_pdf_path.clone()],
    };

    // Attempt rendering pipelines in order of fidelity and speed
    let pdf_generated = try_render_mac_native_pdf(&temp_html_path, &temp_pdf_path)
        .or_else(|_| try_render_chrome_headless_pdf(&temp_html_path, &temp_pdf_path))
        .or_else(|_| try_render_weasyprint(&temp_html_path, &temp_pdf_path))
        .or_else(|_| try_render_wkhtmltopdf(&temp_html_path, &temp_pdf_path));

    match pdf_generated {
        Ok(_) => {
            let pdf_bytes = fs::read(&temp_pdf_path)?;
            Ok(pdf_bytes)
        }
        Err(e) => Err(anyhow!(
            "PDF rendering failed ({}). Ensure Google Chrome/Chromium, Edge, WeasyPrint, or wkhtmltopdf is available in your PATH. (Tip: For 100% zero-dependency styled output, use '--format html' or '--format docx')",
            e
        )),
    }
}

fn try_render_mac_native_pdf(html_path: &Path, pdf_path: &Path) -> Result<()> {
    if !cfg!(target_os = "macos") {
        return Err(anyhow!("Not on macOS"));
    }

    let script = format!(
        r#"
import WebKit, Foundation, AppKit
from PyObjCTools import AppHelper

url = Foundation.NSURL.fileURLWithPath_("{}")
out_path = "{}"

class PrintDelegate(Foundation.NSObject):
    def webView_didFinishLoadForFrame_(self, sender, frame):
        if frame == sender.mainFrame():
            print_info = AppKit.NSPrintInfo.sharedPrintInfo()
            print_info.setPaperSize_((595, 842)) # A4
            print_opts = print_info.dictionary()
            print_opts.setObject_forKey_(Foundation.NSNumber.numberWithBool_(True), AppKit.NSPrintHeaderAndFooter)
            print_opts.setObject_forKey_(out_path, AppKit.NSPrintSavePath)
            print_op = sender.mainFrame().frameView().printOperationWithPrintInfo_(print_info)
            print_op.setShowsPrintPanel_(False)
            print_op.setShowsProgressPanel_(False)
            print_op.runOperation()
            AppKit.NSApplication.sharedApplication().terminate_(None)

view = WebKit.WebView.alloc().init()
delegate = PrintDelegate.alloc().init()
view.setFrameLoadDelegate_(delegate)
view.mainFrame().loadRequest_(Foundation.NSURLRequest.requestWithURL_(url))
AppHelper.runEventLoop()
"#,
        html_path.display(),
        pdf_path.display()
    );

    if let Ok(out) = Command::new("python3").arg("-c").arg(&script).output() {
        if out.status.success() && pdf_path.exists() && fs::metadata(pdf_path)?.len() > 0 {
            return Ok(());
        }
    }

    // Fallback cupsfilter on macOS
    if let Ok(cups_out) = Command::new("cupsfilter").arg(html_path).output() {
        if cups_out.status.success() && !cups_out.stdout.is_empty() {
            fs::write(pdf_path, cups_out.stdout)?;
            return Ok(());
        }
    }

    Err(anyhow!("macOS native print pipeline unavailable"))
}

fn try_render_chrome_headless_pdf(html_path: &Path, pdf_path: &Path) -> Result<()> {
    let chrome_paths = [
        "google-chrome",
        "google-chrome-stable",
        "chromium",
        "chromium-browser",
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/Applications/Brave Browser.app/Contents/MacOS/Brave Browser",
        "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge",
        "/Applications/Chromium.app/Contents/MacOS/Chromium",
        "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
        "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe",
        "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe",
        "msedge",
    ];

    for bin in &chrome_paths {
        let res = Command::new(bin)
            .arg("--headless")
            .arg("--disable-gpu")
            .arg("--no-pdf-header-footer")
            .arg(format!("--print-to-pdf={}", pdf_path.display()))
            .arg(format!("file://{}", html_path.display()))
            .output();

        if let Ok(out) = res {
            if out.status.success()
                && pdf_path.exists()
                && fs::metadata(pdf_path).map(|m| m.len() > 0).unwrap_or(false)
            {
                return Ok(());
            }
        }
    }

    Err(anyhow!("Headless Chromium/Chrome/Edge not found"))
}

fn try_render_weasyprint(html_path: &Path, pdf_path: &Path) -> Result<()> {
    let res = Command::new("weasyprint")
        .arg(html_path)
        .arg(pdf_path)
        .output();

    if let Ok(out) = res {
        if out.status.success() && pdf_path.exists() {
            return Ok(());
        }
    }

    Err(anyhow!("WeasyPrint not found"))
}

fn try_render_wkhtmltopdf(html_path: &Path, pdf_path: &Path) -> Result<()> {
    let res = Command::new("wkhtmltopdf")
        .arg("--quiet")
        .arg("--page-size")
        .arg("A4")
        .arg("--margin-top")
        .arg("15mm")
        .arg("--margin-bottom")
        .arg("15mm")
        .arg("--margin-left")
        .arg("15mm")
        .arg("--margin-right")
        .arg("15mm")
        .arg(html_path)
        .arg(pdf_path)
        .output();

    if let Ok(out) = res {
        if out.status.success() && pdf_path.exists() {
            return Ok(());
        }
    }

    Err(anyhow!("wkhtmltopdf not found"))
}

fn uuid_like_nonce() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}
