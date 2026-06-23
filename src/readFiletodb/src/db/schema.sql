-- statement: createTableIdentify

CREATE TABLE Identify (
    identify text --識別子
  );

-- statement: createTableSampleJsonData

CREATE TABLE SampleJsonData (
    identify text, --識別子
    id text, --ID
    name text, --名称
    type text, --種別
    path text, --パス
    price text, --価格
    category text --カテゴリ
  );

-- statement: createTableAllAddress

CREATE TABLE [AllAddress] (
    [code] VARCHAR(5), --コード
    [oldAddress] VARCHAR(5), --旧郵便番号
    [Address] VARCHAR(7), --郵便番号
    [PrefectureKana] VARCHAR(100), --都道府県カナ
    [MunicipalitiesKana] VARCHAR(100), --市区町村名カナ
    [TownKana] VARCHAR(100), --町域名カナ
    [Prefecture] VARCHAR(100), --都道府県
    [Municipalities] VARCHAR(100), --市区町村名
    [Town] VARCHAR(100), --町域名
    [ChomeMultiFlg] INTEGER, --一町域が二以上の郵便番号で表される場合の表示
    [SmallFlg] INTEGER, --小字毎に番地が起番されている町域の表示
    [ChomeFlg] INTEGER, --丁目を有する町域の場合の表示
    [AddressMultiFlg] INTEGER, --一つの郵便番号で二以上の町域を表す場合の表示
    [updateFlg] INTEGER, --更新の表示（「0」は変更なし、「1」は変更あり、「2」廃止（廃止データのみ使用））
    [changeFlg] INTEGER --変更理由 （「0」は変更なし、「1」市政・区政・町政・分区・政令指定都市施行、「2」住居表示の実施、「3」区画整理、「4」郵便区調整等、「5」訂正、「6」廃止（廃止データのみ使用））
);

-- statement: createTableSampleText

CREATE TABLE SampleText (
    identify text, --識別子
    KeyNum text, --インデックス（キー）
    Div text, --区分
    Num text --番号
  );

-- statement: createTableSampleText2

CREATE TABLE SampleText2 (
    KeyNum text, --インデックス（キー）
    Div text, --区分
    Num text --番号
  );