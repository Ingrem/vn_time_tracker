fn main() {
    // Only run this resource embedding on Windows targets.
    if cfg!(target_os = "windows") {
        // Minimal application manifest.
        const MANIFEST: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <assemblyIdentity version="1.0.0.0" processorArchitecture="*" name="VNTimeTracker" type="win32"/>
  <description>VN Time Tracker</description>
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="asInvoker" uiAccess="false"/>
      </requestedPrivileges>
    </security>
  </trustInfo>
</assembly>
"#;

        let mut res = winres::WindowsResource::new();

        // Application icon.
        res.set_icon("resources/icon.ico");

        // Version and metadata information (visible in file properties).
        res.set("FileDescription", "VN Time Tracker");
        res.set("ProductName", "VN Time Tracker");
        res.set("CompanyName", "Ingrem");
        res.set("LegalCopyright", "© 2025 Ingrem");
        res.set("FileVersion", "1.0.0");
        res.set("ProductVersion", "1.0.0");

        // Attach manifest and compile resources into the final binary.
        res.set_manifest(MANIFEST);
        res.compile().unwrap();
    }
}
