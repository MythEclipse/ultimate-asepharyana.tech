param(
    [string]$Path = ".",
    [string[]]$Exclude = @("node_modules", ".next", "target", ".git", "logs")
)

function Show-Tree {
    param (
        [string]$Folder,
        [string]$Prefix = "",
        [string[]]$Exclude
    )

    $items = Get-ChildItem -LiteralPath $Folder | Sort-Object Name
    $count = $items.Count
    for ($i = 0; $i -lt $count; $i++) {
        $item = $items[$i]
        $isLast = ($i -eq $count - 1)

        if ($Exclude -contains $item.Name) {
            continue
        }

        $connector = if ($isLast) { "\--- " } else { "+--- " }

        if ($item.PSIsContainer) {
            Write-Host "$Prefix$connector$item"
            $newPrefix = if ($isLast) { "$Prefix    " } else { "$Prefix|   " }
            Show-Tree -Folder $item.FullName -Prefix $newPrefix -Exclude $Exclude
        } else {
            Write-Host "$Prefix$connector$item"
        }
    }
}

Show-Tree -Folder (Resolve-Path $Path) -Exclude $Exclude
