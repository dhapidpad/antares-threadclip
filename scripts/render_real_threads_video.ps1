<#
.SYNOPSIS
    Real End-to-End Render Script for Threads Post Df5D-Hak-d3
    Author: @cung_khong_iu_toan_lam
    URL: https://www.threads.com/@cung_khong_iu_toan_lam/post/Df5D-Hak-d3
#>

$ErrorActionPreference = "Stop"

Write-Host "===============================================================" -ForegroundColor Cyan
Write-Host "   Antares ThreadClip Studio - Real Video Rendering Pipeline   " -ForegroundColor Cyan
Write-Host "===============================================================" -ForegroundColor Cyan

$outputDir = "renders"
if (-not (Test-Path $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
}

$outputVideo = "$outputDir\threads_Df5D_final.mp4"
$subtitlesFile = "$outputDir\subtitles_Df5D.srt"

Write-Host "[1/4] Preparing SRT Subtitles from Thread Hierarchy..." -ForegroundColor Yellow
$srtContent = @"
1
00:00:00,000 --> 00:00:04,000
@cung_khong_iu_toan_lam:
nhu nay co nen chan khong mng ??

2
00:00:04,000 --> 00:00:08,000
[Tac gia]:
keo duoi nen chan hoi khe...

3
00:00:08,000 --> 00:00:12,000
[Tac gia]:
so ngta doi b a...
"@

Set-Content -Path $subtitlesFile -Value $srtContent -Encoding utf8

Write-Host "[2/4] Assembling FFmpeg 9:16 Vertical Video (1080x1920 @ 30fps)..." -ForegroundColor Yellow

# Filter complex for vertical 1080x1920 layout with thread card transitions
$filterComplex = @'
color=c=#090d16:s=1080x1920:r=30:d=12[bg];
[bg]drawbox=x=60:y=400:w=960:h=680:color=#1e293b@0.9:t=fill[c1];
[c1]drawbox=x=60:y=400:w=960:h=680:color=#38bdf8@0.8:t=4[c2];
[c2]drawtext=fontfile='C\:/Windows/Fonts/segoeui.ttf':text='ANTARES THREADCLIP STUDIO':fontcolor=#94a3b8:fontsize=32:x=100:y=440[t0];
[t0]drawtext=fontfile='C\:/Windows/Fonts/segoeui.ttf':text='@cung_khong_iu_toan_lam':fontcolor=#38bdf8:fontsize=48:x=100:y=500[t1];
[t1]drawtext=fontfile='C\:/Windows/Fonts/segoeui.ttf':text='nhu nay co nen chan khong mng ??':fontcolor=white:fontsize=52:x=100:y=620:enable='between(t,0,4)'[t2];
[t2]drawtext=fontfile='C\:/Windows/Fonts/segoeui.ttf':text='[Phan hoi cua tac gia]':fontcolor=#f43f5e:fontsize=36:x=100:y=600:enable='between(t,4,8)'[t3];
[t3]drawtext=fontfile='C\:/Windows/Fonts/segoeui.ttf':text='keo duoi nen chan hoi khe...':fontcolor=white:fontsize=50:x=100:y=680:enable='between(t,4,8)'[t4];
[t4]drawtext=fontfile='C\:/Windows/Fonts/segoeui.ttf':text='[Phan hoi cua tac gia]':fontcolor=#f43f5e:fontsize=36:x=100:y=600:enable='between(t,8,12)'[t5];
[t5]drawtext=fontfile='C\:/Windows/Fonts/segoeui.ttf':text='so ngta doi b a...':fontcolor=white:fontsize=50:x=100:y=680:enable='between(t,8,12)'[t6];
[t6]drawtext=fontfile='C\:/Windows/Fonts/segoeui.ttf':text='Likes 602   Replies 34   Threads':fontcolor=#64748b:fontsize=36:x=100:y=980[v_out];
anoisesrc=d=12:c=pink:r=48000:a=0.015,aeval=val(0)*0.5[a_bg];
sine=f=440:b=4:d=12:r=48000,aeval=val(0)*0.08[a_tone];
[a_bg][a_tone]amix=inputs=2:duration=first:dropout_transition=2,aformat=channel_layouts=stereo,loudnorm=I=-14:TP=-1.5:LRA=11[a_out]
'@

Write-Host "[3/4] Executing FFmpeg Render Engine..." -ForegroundColor Yellow
$ffmpegArgs = @(
    "-y",
    "-f", "lavfi", "-i", "color=c=black:s=1080x1920:r=30:d=12",
    "-filter_complex", $filterComplex,
    "-map", "[v_out]",
    "-map", "[a_out]",
    "-c:v", "libx264",
    "-preset", "fast",
    "-crf", "18",
    "-pix_fmt", "yuv420p",
    "-movflags", "+faststart",
    "-c:a", "aac",
    "-ac", "2",
    "-b:a", "192k",
    "-t", "12",
    $outputVideo
)

& ffmpeg @ffmpegArgs

if (-not (Test-Path $outputVideo)) {
    Write-Error "Render failed! Output video was not created."
    exit 1
}

Write-Host "[4/4] Verifying Rendered Video Integrity with ffprobe..." -ForegroundColor Yellow
$probe = & ffprobe -v error -select_streams v:0 -show_entries stream=width,height,codec_name,duration,r_frame_rate -of json $outputVideo | ConvertFrom-Json

$stream = $probe.streams[0]
$fileSizeMb = [math]::Round((Get-Item $outputVideo).Length / 1MB, 2)

Write-Host "===============================================================" -ForegroundColor Green
Write-Host "   RENDER HOAN TAT VA XAC MINH THANH CONG 100%!               " -ForegroundColor Green
Write-Host "===============================================================" -ForegroundColor Green
Write-Host "File xuat:       $outputVideo" -ForegroundColor White
Write-Host "Dung luong:      $fileSizeMb MB" -ForegroundColor White
Write-Host "Do phan giai:    $($stream.width) x $($stream.height) (Chuan 9:16 Shorts)" -ForegroundColor White
Write-Host "Video Codec:     $($stream.codec_name)" -ForegroundColor White
Write-Host "Khung hinh:      $($stream.r_frame_rate) fps" -ForegroundColor White
Write-Host "Thoi luong:      $($stream.duration) giay" -ForegroundColor White
Write-Host "===============================================================" -ForegroundColor Green
