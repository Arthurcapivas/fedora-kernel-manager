use adw::prelude::ActionRowExt;
use gtk::{Align, IconSize, Orientation, SelectionMode, SizeGroupMode};
use gtk::prelude::{BoxExt, WidgetExt};
use std::process::{Command, Stdio};
use crate::{KernelBranch, RunningKernelInfo};
use Vec;

pub fn content() -> gtk::Box {

    let running_kernel_info = get_running_kernel_info();

    let content_box = gtk::Box::builder()
        .hexpand(true)
        .vexpand(true)
        .orientation(Orientation::Vertical)
        .build();

    let tux_icon = gtk::Image::builder()
        .pixel_size(128)
        .halign(Align::Center)
        .hexpand(true)
        .margin_start(10)
        .margin_end(10)
        .margin_bottom(20)
        .margin_top(20)
        .build();

    tux_icon.set_icon_name(Some("tux-symbolic"));

    tux_icon.add_css_class("symbolic-accent-bg");

    let kernel_badge_box = gtk::Box::builder()
        .hexpand(true)
        .vexpand(true)
        .orientation(Orientation::Vertical)
        .build();

    create_kernel_badges(&kernel_badge_box, &running_kernel_info);

    content_box.append(&kernel_badge_box);
    content_box.append(&tux_icon);

    content_box
}

fn create_kernel_badge(label0_text: &str, label1_text: &str, css_style: &str, group_size: &gtk::SizeGroup, group_size0: &gtk::SizeGroup, group_size1: &gtk::SizeGroup) -> gtk::ListBox {
    let badge_box = gtk::Box::builder()
        .build();

    let label0 = gtk::Label::builder()
        .label(label0_text)
        .margin_start(5)
        .margin_end(5)
        .margin_bottom(1)
        .margin_top(1)
        .hexpand(true)
        .vexpand(true)
        .build();
    group_size0.add_widget(&label0);

    let label_seprator = gtk::Separator::builder()
        .build();

    let label1 = gtk::Label::builder()
        .label(label1_text)
        .margin_start(3)
        .margin_end(0)
        .margin_bottom(1)
        .margin_top(1)
        .hexpand(true)
        .vexpand(true)
        .build();
    group_size1.add_widget(&label1);

    label1.add_css_class(css_style);

    badge_box.append(&label0);
    badge_box.append(&label_seprator);
    badge_box.append(&label1);

    let boxedlist = gtk::ListBox::builder()
        .selection_mode(SelectionMode::None)
        .halign(Align::Center)
        .valign(Align::End)
        .margin_start(5)
        .margin_end(5)
        .margin_bottom(5)
        .margin_top(5)
        .build();

    boxedlist.add_css_class("boxed-list");
    boxedlist.append(&badge_box);
    group_size.add_widget(&boxedlist);
    boxedlist
}

//fn get_kernel_branches() -> Vec<KernelBranch> {}
fn get_running_kernel_info() -> RunningKernelInfo {
    let kernel = match Command::new("uname").arg("-r").stdout(Stdio::piped()).output() {
        Ok(t) =>  String::from_utf8(t.stdout).unwrap().trim().to_owned(),
        Err(_) => "Unknown".to_string()
    };

    let version = match linux_version::linux_kernel_version() {
        Ok(t) => format!("{}.{}.{}", t.major, t.minor, t.patch),
        Err(_) => "Unknown".to_string()
    };

    let info = RunningKernelInfo {
        kernel: kernel,
        version: version,
        // didn't find a way to accurately get this, outside of sched-ext (https://github.com/CachyOS/kernel-manager/blob/develop/src/schedext-window.cpp)
        sched: "TODO".to_owned()
    };

    info
}

fn create_kernel_badges(badge_box: &gtk::Box, running_kernel_info: &RunningKernelInfo) {
    let kernel_badges_size_group = gtk::SizeGroup::new(SizeGroupMode::Both);
    let kernel_badges_size_group0 = gtk::SizeGroup::new(SizeGroupMode::Both);
    let kernel_badges_size_group1 = gtk::SizeGroup::new(SizeGroupMode::Both);

    let kernel_version = "6.9";

    let version_css_style = if &running_kernel_info.version.as_str() == &kernel_version {
        "background-green-bg"
    }
    else {
        "background-red-bg"
    };

    badge_box.append(&create_kernel_badge("Kernel Branch", "cachy", "background-accent-bg", &kernel_badges_size_group, &kernel_badges_size_group0, &kernel_badges_size_group1));
    badge_box.append(&create_kernel_badge("Latest Version", "6.9", "background-accent-bg", &kernel_badges_size_group, &kernel_badges_size_group0, &kernel_badges_size_group1));
    badge_box.append(&create_kernel_badge("Running Version", &running_kernel_info.version, &version_css_style, &kernel_badges_size_group, &kernel_badges_size_group0, &kernel_badges_size_group1));
    badge_box.append(&create_kernel_badge("Running Kernel", &running_kernel_info.kernel, &version_css_style, &kernel_badges_size_group, &kernel_badges_size_group0, &kernel_badges_size_group1));
    badge_box.append(&create_kernel_badge("Running Sched", &running_kernel_info.sched, "background-accent-bg", &kernel_badges_size_group, &kernel_badges_size_group0, &kernel_badges_size_group1));
}