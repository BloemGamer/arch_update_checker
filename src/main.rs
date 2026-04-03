// Correctness / Safety
#![warn(clippy::absurd_extreme_comparisons)]
#![warn(clippy::bool_assert_comparison)]
#![warn(clippy::cast_ptr_alignment)]
#![warn(clippy::cast_slice_from_raw_parts)]
#![warn(clippy::collapsible_if)]
#![warn(clippy::cmp_owned)]
#![warn(clippy::eq_op)]
#![warn(clippy::float_cmp)]
#![warn(clippy::float_cmp_const)]
#![warn(clippy::identity_op)]
#![warn(clippy::implicit_clone)]
#![warn(clippy::implicit_hasher)]
#![warn(clippy::infallible_destructuring_match)]
#![warn(clippy::invalid_upcast_comparisons)]
#![warn(clippy::if_same_then_else)]
#![warn(clippy::manual_assert)]
#![warn(clippy::manual_memcpy)]
#![warn(clippy::manual_range_contains)]
#![warn(clippy::match_like_matches_macro)]
#![warn(clippy::match_same_arms)]
#![warn(clippy::multiple_unsafe_ops_per_block)]
#![warn(clippy::op_ref)]
#![warn(clippy::ptr_as_ptr)]
#![warn(clippy::ptr_offset_with_cast)]
#![warn(clippy::redundant_pattern_matching)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::same_item_push)]
#![warn(clippy::single_match)]
#![warn(clippy::trait_duplication_in_bounds)]
#![warn(clippy::transmute_ptr_to_ptr)]
#![warn(clippy::unchecked_time_subtraction)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::unused_async)]
#![warn(clippy::useless_conversion)]
#![warn(clippy::while_let_loop)]
#![warn(clippy::zero_prefixed_literal)]
// Error Handling
#![warn(clippy::fallible_impl_from)]
#![warn(clippy::from_over_into)]
#![warn(clippy::manual_ok_or)]
#![warn(clippy::manual_unwrap_or)]
#![warn(clippy::panic)]
#![warn(clippy::unnecessary_wraps)]
#![cfg_attr(test, allow(clippy::panic, clippy::unwrap_used))]
// Performance / Allocation
#![warn(clippy::borrowed_box)]
#![warn(clippy::box_collection)]
#![warn(clippy::large_stack_arrays)]
#![warn(clippy::naive_bytecount)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::needless_collect)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::or_fun_call)]
#![warn(clippy::rc_buffer)]
#![warn(clippy::rc_mutex)]
#![warn(clippy::redundant_allocation)]
#![warn(clippy::redundant_clone)]
#![warn(clippy::slow_vector_initialization)]
#![warn(clippy::trivially_copy_pass_by_ref)]
#![warn(clippy::inefficient_to_string)]
#![warn(clippy::result_large_err)]
#![warn(clippy::useless_vec)]
#![warn(clippy::vec_box)]
#![warn(clippy::unsound_collection_transmute)]
#![warn(clippy::needless_range_loop)]
#![warn(clippy::iter_skip_zero)]
// Iterator
#![warn(clippy::manual_flatten)]
#![warn(clippy::manual_map)]
#![warn(clippy::needless_for_each)]
// Control Flow / Code Structure
#![warn(clippy::branches_sharing_code)]
#![warn(clippy::match_bool)]
#![warn(clippy::match_wildcard_for_single_variants)]
#![warn(clippy::never_loop)]
#![warn(clippy::redundant_guards)]
#![warn(clippy::unnested_or_patterns)]
// Style
#![warn(clippy::enum_glob_use)]
#![warn(clippy::implicit_return)]
#![warn(clippy::items_after_statements)]
#![warn(clippy::let_underscore_untyped)]
#![warn(clippy::mixed_read_write_in_expression)]
#![warn(clippy::module_name_repetitions)]
#![warn(clippy::redundant_static_lifetimes)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
// #![warn(clippy::self_named_module_files)]
#![warn(clippy::shadow_reuse)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![warn(clippy::style)]
#![warn(clippy::verbose_bit_mask)]
// Docs
#![warn(clippy::doc_markdown)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::missing_safety_doc)]
// Warning Collections
#![warn(clippy::nursery)]
// Cargo
// #![warn(clippy::cargo)]
// #![warn(clippy::cargo_common_metadata)]

// Allow
#![allow(clippy::needless_return)]
#![allow(clippy::use_self)]
#![allow(clippy::result_large_err)] // TODO: in the future, maybe fix all of them
#![allow(clippy::self_named_module_files)]
// #![allow(dead_code)]

// #![warn(clippy::todo)]

use clap::Parser;
use serde_json::json;
use std::{fmt::Write, io};

#[derive(Debug, Clone, PartialEq, Default)]
struct UpdateManager
{
	pacman: bool,
	flatpak: bool,
	updates: Vec<Update>,
}

#[derive(Debug, Clone, PartialEq)]
struct Update
{
	name: String,
	source: UpdateSource,
	version: Versions,
}

#[derive(Debug, Clone, PartialEq)]
struct Versions
{
	old: Option<Version>,
	new: Option<Version>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UpdateSource
{
	Pacman,
	Flatpak,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct Version(String);

#[derive(Debug, Clone, PartialEq, clap::Parser)]
#[command(version, about, long_about = None)]
struct Config
{
	#[arg(short, long)]
	pacman: bool,
	#[arg(short, long)]
	flatpak: bool,
	#[arg(short, long)]
	update_command: bool,
}

impl UpdateManager
{
	fn fetch_pacman(mut self) -> io::Result<Self>
	{
		let status: std::process::Output = std::process::Command::new("checkupdates").output()?;
		if status.status.code() == Some(2) {
			// if it returns 2, it has no updates, so we can just return
			return Ok(self);
		}

		let output: String =
			String::from_utf8(status.stdout).expect("I don't thing checkupdates does return not utf8 output");
		let mut up: Vec<Update> = output
			.split('\n')
			.filter_map(|s| {
				if s.is_empty() {
					return None;
				}
				let tmp: (&str, &str) = s.split_once("->").expect("this should just do it");
				let (name, old_version): (&str, &str) = tmp.0.split_once(' ').expect("this should just do it 2");
				return Some(Update {
					name: name.into(),
					source: UpdateSource::Pacman,
					version: Versions {
						old: Some(Version(old_version.trim().to_string())),
						new: Some(Version(tmp.1.trim().to_string())),
					},
				});
			})
			.collect();

		if !up.is_empty() {
			self.updates.append(&mut up);
			self.pacman = true;
		}

		return Ok(self);
	}

	fn fetch_flatpak(mut self) -> io::Result<Self>
	{
		let status: std::process::Output = std::process::Command::new("flatpak")
			.args(["remote-ls", "--updates", "--columns", "name,version"])
			.output()?;
		let output: String =
			String::from_utf8(status.stdout).expect("I don't thing checkupdates does return not utf8 output");

		let mut up: Vec<Update> = output
			.split('\n')
			.filter_map(|s| {
				if s.is_empty() {
					return None;
				}

				let parts: Vec<&str> = s.split('\t').filter(|st| return !st.is_empty()).collect();

				let name: &str = parts[0];
				let version: &str = parts.get(1).unwrap_or(&"").trim();

				return Some(Update {
					name: name.to_string(),
					source: UpdateSource::Flatpak,
					version: Versions {
						old: None,
						new: if !version.is_empty() {
							Some(Version(version.to_string()))
						} else {
							None
						},
					},
				});
			})
			.collect();

		if !up.is_empty() {
			self.updates.append(&mut up);
			self.flatpak = true;
		}

		return Ok(self);
	}

	fn to_waybar(&self) -> serde_json::Value
	{
		let mut pacman: Vec<&Update> = Vec::new();
		let mut flatpak: Vec<&Update> = Vec::new();

		for update in &self.updates {
			match update.source {
				UpdateSource::Pacman => pacman.push(update),
				UpdateSource::Flatpak => {
					// filter junk entries
					if update.name != "default" {
						flatpak.push(update);
					}
				}
			}
		}

		let total: usize = pacman.len() + flatpak.len();

		if total == 0 {
			return json!({
				"text": "",
				"tooltip": "No updates",
				"class": "ok"
			});
		}

		let mut tooltip = String::new();

		if !pacman.is_empty() {
			writeln!(tooltip, " Pacman ({})", pacman.len()).ok();

			for u in pacman {
				let version = match (&u.version.old, &u.version.new) {
					(Some(old), Some(new)) => format!("{} → {}", old.0, new.0),
					_ => String::new(),
				};

				if version.is_empty() {
					writeln!(tooltip, "  {}", u.name).ok();
				} else {
					writeln!(tooltip, "  {} {}", u.name, version).ok();
				}
			}

			writeln!(tooltip).ok();
		}

		if !flatpak.is_empty() {
			writeln!(tooltip, " Flatpak ({})", flatpak.len()).ok();

			for u in flatpak {
				writeln!(tooltip, "  {}", u.name).ok();
			}
		}

		let class: &str = match total {
			0 => "ok",
			1..=4 => "low",
			5..=14 => "medium",
			_ => "high",
		};

		return json!({
			"text": format!("󰏗 {}", total),
			"tooltip": tooltip,
			"class": class
		});
	}
}

fn main()
{
	let config: Config = Config::parse();
	let mut updates: UpdateManager = UpdateManager::default();

	if config.pacman {
		updates = updates.fetch_pacman().unwrap();
	}

	if config.flatpak {
		updates = updates.fetch_flatpak().unwrap();
	}

	if config.update_command {
		let mut first: bool = true;
		if updates.pacman {
			if !first {
				print!(" && ");
			}
			first = false;
			println!("pacman -Syu");
		}
		if updates.flatpak {
			if !first {
				print!(" && ");
			}
			first = false;
			println!("flatpak update");
		}
		let _: bool = first;
	} else {
		println!("{}", updates.to_waybar());
	}
}
