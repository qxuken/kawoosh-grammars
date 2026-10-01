#Requires -Version 7.0
<#
.SYNOPSIS
A sample.
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [string]$Name = "world",
    [int]$Limit = 10
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

class Point {
    [double]$X
    [double]$Y

    Point([double]$x, [double]$y) {
        $this.X = $x
        $this.Y = $y
    }

    [double] Length() {
        return [math]::Sqrt($this.X * $this.X + $this.Y * $this.Y)
    }
}

function Get-Largest {
    param([int[]]$Items)
    if ($Items.Count -eq 0) { return $null }
    $best = $Items[0]
    foreach ($item in $Items) {
        if ($item -gt $best) { $best = $item }
    }
    return $best
}

$points = @([Point]::new(3, 4), [Point]::new(1, 0))
$lengths = $points | ForEach-Object { $_.Length() } | Where-Object { $_ -gt 1 }
$table = @{ long = $lengths.Count; all = $points.Count }

# A comment.
foreach ($key in $table.Keys) {
    Write-Host "${key}: $($table[$key])"
}

try {
    $n = [int]::Parse("42")
    Write-Output "Hello $Name, $(Get-Largest -Items 1, 2, 3) of $($n + $Limit)"
}
catch [System.FormatException] {
    Write-Error "not a number"
}
finally {
    switch ($Limit) {
        10 { "ten" }
        default { "other" }
    }
}
