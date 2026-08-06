# Video Analysis Tooling — Research & Recommendation

Survey of open-source GitHub projects for video analysis, evaluated for the
Alien Life Wisdom / Geneva's Log workflow: 8–15 second AI-generated clips,
a ~4 hour per video budget, and Claude Code as the working environment.

Researched 2026-08-06. Star counts and activity dates are from the GitHub API
on that date.

---

## Recommendation

**[HUANGCHIHHUNGLeo/claude-real-video](https://github.com/HUANGCHIHHUNGLeo/claude-real-video)**
— MIT, Python, ~1,960 stars, last commit 2026-08-03.

It extracts scene-aware, deduplicated keyframes plus a transcript from a local
file *or* a URL, and hands them to Claude as images with a timestamp manifest.
That is the whole job. Claude does the seeing; the tool solves the boring part
(which frames actually matter, and how to cite them back).

Why it wins for this channel specifically:

- **It ingests URLs, not just files.** yt-dlp underneath means TikTok, Reels,
  and YouTube go in directly. Studying why a Zach King or King Bach opener
  works is the same command as checking your own export — no manual downloading
  in between.
- **No API key, no GPU, no model download** for the core keyframe path. ffmpeg
  and Python 3.10+. Whisper is an optional extra you can skip entirely, which
  matters because a 15-second mute-first clip has almost no dialogue worth
  transcribing.
- **Runs locally.** The video never leaves the machine unless you paste frames
  into a cloud model yourself.
- **It's already a Claude Code skill**, so the analysis happens where the rest
  of the work happens rather than in a separate app.
- **Actively maintained.** v0.8.0 in August 2026 added an MCP server; the
  changelog reads like someone using it on real footage (dedup fixes from a
  2,181-video batch report, anti-hallucination gating on transcripts).

The honest caveat: a paid Pro tier ($29 one-time) gates camera-motion analysis,
audio emotion curves, and AI-generated cinematography reports. The free core —
keyframes, dedup, transcript, local processing — is complete and is the part
that matters here. Don't buy Pro until the free path proves itself.

### Tuning it for 8–15 second clips

The defaults assume long video. A 15-second clip needs the opposite of
aggressive deduplication — dense sampling, especially at the front.

```bash
pip install "claude-real-video[whisper]"        # drop [whisper] if skipping audio
npx skills add HUANGCHIHHUNGLeo/claude-real-video   # Claude Code skill
```

```bash
# Hook audit — dense frames, contact sheet, focused question
crv ./geneva-ep12.mp4 --fps-floor 0.25 --adaptive --grid \
    --why "Does the first frame read as mid-action with the sound off?"
```

- `--fps-floor 0.25` forces a frame every quarter-second (~60 frames for a
  15s clip) instead of the default one per second. Scene detection alone will
  under-sample a clip this short.
- `--adaptive` compares each frame against its rolling neighbourhood rather
  than a fixed threshold. This is the flag that matters for Geneva — AI
  generation tends to *morph* gradually rather than cut, and a fixed threshold
  misses a slow tendril movement that never spikes any single frame.
- `--grid` produces 3×3 contact sheets. Useful for judging mute-legibility the
  way the Chicago-office-lunch-break viewer actually sees it: at a glance,
  half-distracted, no sound.

### Where it maps onto the playbook

| Playbook need | How this helps |
|---|---|
| Frame 1 must be mid-action and mute-legible (§8) | Dense front sampling + contact sheet; ask Claude to judge frame 1 cold, without the premise |
| Drop before the reveal → flat escalation (§8) | Timestamped frames let you point at the exact beat that goes slack |
| Continuity ledger (§7) | Run over past episodes to recover location, glow state, and which rare-parts objects are actually in frame |
| Anti-slop: recurrence over randomness (§6) | Frame-level check that the ship, the pile, and the tendrils really appear where you think they do |

No tool measures "mute-legibility" — that is Claude's judgment on the frames.
What the tool provides is the right frames to judge.

---

## Runner-ups

**[jordanrendric/claude-video-vision](https://github.com/jordanrendric/claude-video-vision)**
— MIT, TypeScript, ~1,100 stars. A Claude Code plugin with a clean MCP server
and a `/watch-video` slash command; the smoothest install of the group. Held
back by being v1.0.0 and, per its own README, tested only on macOS Apple
Silicon with the local Whisper backend. Its audio path wants a Gemini or OpenAI
key unless you install `whisper-cpp`. Worth revisiting once it has more
mileage — the ergonomics are genuinely better.

**[oxbshw/watch-skill](https://github.com/oxbshw/watch-skill)**
— MIT, Python, ~260 stars. Heavier: it builds a *persistent, searchable index*
across sessions with timestamped citations. Overkill for one 15-second clip,
but it is the closest thing to an automated continuity ledger — a queryable
archive of every episode you have shipped. Consider it later, if the ledger
becomes the bottleneck.

**[Breakthrough/PySceneDetect](https://github.com/Breakthrough/PySceneDetect)**
— ~5,070 stars, maintained since 2014, still updated. The foundational
scene-cut library rather than an analysis product; several tools above build on
this idea. Reach for it only to write a custom script — for example, batch
exporting frame 1 of every episode into one sheet. Not a starting point.

## Ruled out, and why

- **[HKUDS/VideoAgent](https://github.com/HKUDS/VideoAgent)** (~1,660 stars) —
  capable all-in-one understanding/editing agent, but wants an 8GB+ GPU, a
  conda environment, six model downloads, and keys for four providers. That is
  a weekend, and the budget is four hours per video.
- **[NVIDIA-AI-Blueprints/video-search-and-summarization](https://github.com/NVIDIA-AI-Blueprints/video-search-and-summarization)** —
  enterprise GPU reference architecture for real-time video analytics. Built
  for surveillance-scale workloads, not 15-second comedy.
- **Research toolkits** — [mmaction2](https://github.com/open-mmlab/mmaction2)
  (~5,120), [InternVideo](https://github.com/OpenGVLab/InternVideo) (~2,350),
  [VideoMAE](https://github.com/MCG-NJU/VideoMAE) (~1,780),
  [PaddleVideo](https://github.com/PaddlePaddle/PaddleVideo) (~1,700). These do
  action recognition and classification against benchmark datasets. They answer
  "which of 400 labeled actions is this," not "is this funny in the first three
  seconds." Wrong tier of question.

## What does not exist

There is no open-source tool that analyzes short-form *hooks* or first-frame
retention. Everything marketed that way is a commercial hook-line generator,
and none of it looks at your actual footage. The working approach is the one
above: pull the right frames, then ask a capable model the right question. The
judgment stays with you and Claude — the tooling just makes the frames cheap to
look at.
