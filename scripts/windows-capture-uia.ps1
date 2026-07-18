param(
    [string]$OutputPath = (Join-Path (Get-Location) "microdeck-uia-snapshot.json"),
    [ValidateRange(1, 20)]
    [int]$MaxDepth = 10,
    [ValidateRange(1, 1000)]
    [int]$MaxChildrenPerNode = 250
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes

# Uses AutomationElement.FromHandle through the .NET UI Automation wrapper.
# The capture intentionally excludes absolute geometry properties and all screen coordinates.

function Convert-ToSafeText {
    param([AllowNull()][object]$Value)

    if ($null -eq $Value) {
        return ""
    }

    $text = [string]$Value
    $text = $text -replace '(?i)C:\\Users\\[^\\\s]+', 'C:\Users\<redacted>'
    $text = $text -replace '(?i)Bearer\s+[A-Za-z0-9._~+\-/]+=*', 'Bearer <redacted>'
    $text = $text -replace '(?i)sk-[A-Za-z0-9_-]{8,}', 'sk-<redacted>'

    if ($text.Length -gt 180) {
        return $text.Substring(0, 180) + "…"
    }

    return $text
}

function Get-SafeProcessVersion {
    param([System.Diagnostics.Process]$Process)

    try {
        return $Process.MainModule.FileVersionInfo.FileVersion
    }
    catch {
        return $null
    }
}

function Get-TargetProcess {
    $processCandidates = @("ChatGPT", "Codex")
    $titleCandidates = @("ChatGPT", "Codex")

    $visible = @(Get-Process | Where-Object { $_.MainWindowHandle -ne 0 })

    foreach ($candidate in $processCandidates) {
        $match = $visible |
            Where-Object { $_.ProcessName -ieq $candidate } |
            Sort-Object StartTime -Descending |
            Select-Object -First 1
        if ($null -ne $match) {
            return $match
        }
    }

    foreach ($candidate in $titleCandidates) {
        $match = $visible |
            Where-Object { $_.MainWindowTitle -like "*$candidate*" } |
            Sort-Object StartTime -Descending |
            Select-Object -First 1
        if ($null -ne $match) {
            return $match
        }
    }

    return $null
}

function Get-SupportedPatternNames {
    param([System.Windows.Automation.AutomationElement]$Element)

    try {
        return @($Element.GetSupportedPatterns() | ForEach-Object {
            Convert-ToSafeText $_.ProgrammaticName
        })
    }
    catch {
        return @()
    }
}

$walker = [System.Windows.Automation.TreeWalker]::ControlViewWalker

function Convert-UiaElement {
    param(
        [System.Windows.Automation.AutomationElement]$Element,
        [int]$Depth
    )

    if ($Depth -gt $MaxDepth) {
        return $null
    }

    try {
        $current = $Element.Current
        $name = Convert-ToSafeText $current.Name
        $automationId = Convert-ToSafeText $current.AutomationId
        $controlType = if ($null -ne $current.ControlType) {
            Convert-ToSafeText $current.ControlType.ProgrammaticName
        }
        else {
            ""
        }
        $className = Convert-ToSafeText $current.ClassName
        $isEnabled = [bool]$current.IsEnabled
        $isOffscreen = [bool]$current.IsOffscreen
    }
    catch {
        return $null
    }

    $children = [System.Collections.Generic.List[object]]::new()

    if ($Depth -lt $MaxDepth) {
        try {
            $child = $walker.GetFirstChild($Element)
            $count = 0
            while ($null -ne $child -and $count -lt $MaxChildrenPerNode) {
                $converted = Convert-UiaElement -Element $child -Depth ($Depth + 1)
                if ($null -ne $converted) {
                    $children.Add($converted)
                }
                $child = $walker.GetNextSibling($child)
                $count++
            }
        }
        catch {
            # A stale or protected subtree should not invalidate the rest of the capture.
        }
    }

    return [ordered]@{
        name = $name
        automationId = $automationId
        controlType = $controlType
        className = $className
        isEnabled = $isEnabled
        isOffscreen = $isOffscreen
        patterns = @(Get-SupportedPatternNames -Element $Element)
        children = @($children)
    }
}

$target = Get-TargetProcess
if ($null -eq $target) {
    throw "MicroDeck could not find a visible ChatGPT or Codex desktop window. Open the app and Codex experience, then run this script again."
}

# [System.Windows.Automation.AutomationElement]::FromHandle is the concrete AutomationElement.FromHandle call.
$rootElement = [System.Windows.Automation.AutomationElement]::FromHandle($target.MainWindowHandle)
if ($null -eq $rootElement) {
    throw "Windows UI Automation did not expose a root element for the detected target window."
}

Write-Host "Capturing UI Automation tree from $($target.ProcessName) — $($target.MainWindowTitle)"
Write-Host "MaxDepth=$MaxDepth MaxChildrenPerNode=$MaxChildrenPerNode"

$root = Convert-UiaElement -Element $rootElement -Depth 0
if ($null -eq $root) {
    throw "The target window was found, but its UI Automation root could not be read. Check privilege levels and try again."
}

$document = [ordered]@{
    schemaVersion = 1
    capturedAt = [DateTimeOffset]::UtcNow.ToString("o")
    target = [ordered]@{
        processName = "$($target.ProcessName).exe"
        processId = $target.Id
        windowTitle = Convert-ToSafeText $target.MainWindowTitle
        processVersion = Get-SafeProcessVersion -Process $target
    }
    root = $root
}

$parent = Split-Path -Parent $OutputPath
if ($parent -and -not (Test-Path $parent)) {
    New-Item -ItemType Directory -Force -Path $parent | Out-Null
}

$document | ConvertTo-Json -Depth 100 | Set-Content -Path $OutputPath -Encoding UTF8
Write-Host "Saved sanitized UI Automation snapshot to: $OutputPath"
Write-Host "Review the JSON before sharing it. MicroDeck truncates text and redacts common user paths/tokens, but accessibility names can still contain app-visible text."
