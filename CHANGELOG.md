# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.0.2](https://github.com/FlakM/librus-rs/compare/v2.0.1...v2.0.2) - 2026-11-02

### Added

- typed text and descriptive grade endpoints

### Fixed

- keep text grades visible when ShowInGradesView is absent
- match live text grade payloads
- fix auth: start from portalRodzina to get oauth_state, enabling oauth_token session
- OAuth grant now requires POST instead of GET
- handle integer IDs in school notices API response

### Other

- follow goTo redirect from login response instead of hardcoded grant URL

### Added

- Text grade support: `base_text_grades`, `text_grades`,
  `descriptive_text_grades`, `text_grade_categories`, `text_grade_category`,
  `descriptive_grade_skills` and `descriptive_grade_skill`, with fully typed
  `TextGrade`, `TextGradeCategory` and `DescriptiveGradeSkill` models.

## [2.0.1](https://github.com/FlakM/librus-rs/compare/v2.0.0...v2.0.1) - 2026-02-02

### Other

- fmt
- Add paged school notices support and docs
- notices
