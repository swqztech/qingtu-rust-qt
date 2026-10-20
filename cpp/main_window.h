#pragma once

#include <QMainWindow>
#include <QStringList>
#include <vector>
#include <string>

class QListWidget;
class QSpinBox;
class QComboBox;
class QCheckBox;
class QLineEdit;
class QTextEdit;
class QPushButton;

class MainWindow : public QMainWindow {
  Q_OBJECT
 public:
  explicit MainWindow(QWidget* parent = nullptr);

 private slots:
  void onAddFiles();
  void onClear();
  void onCompress();
  void onStitch();

 private:
  std::vector<std::string> pathStrings() const;
  void appendLog(const QString& text);

  QStringList files_;
  QListWidget* list_ = nullptr;
  QSpinBox* quality_ = nullptr;
  QSpinBox* max_side_ = nullptr;
  QComboBox* format_ = nullptr;
  QCheckBox* custom_out_ = nullptr;
  QLineEdit* out_dir_ = nullptr;
  QTextEdit* log_ = nullptr;
};
