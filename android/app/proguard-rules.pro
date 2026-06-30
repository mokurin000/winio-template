# Alternatively, you can disable `minifyEnabled` entirely.

# Your application package and Winio Activity
-keep, includedescriptorclasses class io.github.mokurin000.example.** { *; }
-keep, includedescriptorclasses class rs.compio.winio.Activity { *; }
-keep, includedescriptorclasses class com.google.androidgamesdk.** { *; }
-keep, includedescriptorclasses class androidx.activity.result.** { *; }

# Button
-keep, includedescriptorclasses class com.google.android.material.button.** { *; }

# CheckBox
# -keep, includedescriptorclasses class com.google.android.material.checkbox.** { *; }

# RadioButton
# -keep, includedescriptorclasses class com.google.android.material.radiobutton.** { *; }

# Media
# -keep, includedescriptorclasses class androidx.media3.** { *; }

# LinkLabel
# -keep, includedescriptorclasses class rs.compio.winio.ClickableSpan { *; }

# TabView
# -keep, includedescriptorclasses class rs.compio.winio.TabViewAdapter { *; }
# -keep, includedescriptorclasses class com.google.android.material.tabs.** { *; }
# -keep, includedescriptorclasses class androidx.viewpager2.widget.** { *; }

# WebView
# -keep, includedescriptorclasses class rs.compio.winio.WebViewClient { *; }

# Progress
# -keep, includedescriptorclasses class com.google.android.material.progressindicator.** { *; }

# Slider
# -keep, includedescriptorclasses class com.google.android.material.slider.** { *; }