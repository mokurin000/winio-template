# Alternatively, you can disable `minifyEnabled` entirely.

# Your application package and Winio Activity
-keep, includedescriptorclasses class io.github.mokurin000.example.** { *; }
-keep, includedescriptorclasses class rs.compio.winio.Activity { *; }
-keep, includedescriptorclasses class com.google.androidgamesdk.** { *; }
-keep, includedescriptorclasses class androidx.activity.result.** { *; }

# Media
# Uncomment if using the Media component.
# -keep, includedescriptorclasses class androidx.media3.** { *; }

# LinkLabel
# Uncomment if using the LinkLabel component.
# -keep, includedescriptorclasses class rs.compio.winio.ClickableSpan { *; }

# TabView
# Uncomment if using the TabView component.
# -keep, includedescriptorclasses class rs.compio.winio.TabViewAdapter { *; }
# -keep, includedescriptorclasses class com.google.android.material.tabs.** { *; }
# -keep, includedescriptorclasses class androidx.viewpager2.widget.** { *; }

# WebView
# Uncomment if using the WebView component.
# -keep, includedescriptorclasses class rs.compio.winio.WebViewClient { *; }

# Button
# Uncomment if using the Button component.
# -keep, includedescriptorclasses class com.google.android.material.button.** { *; }

# CheckBox
# Uncomment if using the CheckBox component.
# -keep, includedescriptorclasses class com.google.android.material.checkbox.** { *; }

# RadioButton
# Uncomment if using the RadioButton component.
# -keep, includedescriptorclasses class com.google.android.material.radiobutton.** { *; }

# Progress
# Uncomment if using the Progress component.
# -keep, includedescriptorclasses class com.google.android.material.progressindicator.** { *; }

# Slider
# Uncomment if using the Slider component.
# -keep, includedescriptorclasses class com.google.android.material.slider.** { *; }