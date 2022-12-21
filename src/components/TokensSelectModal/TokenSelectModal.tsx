import React, { createRef, useState } from "react";
import useOnClickOutside from "@src/hooks/useOnClickOutside";
import Dialog from "@components/Dialog";
import Scrollbar from "@src/components/Scrollbar";
import { Column } from "@src/components/Flex";
import SizedBox from "@components/SizedBox";
import { observer } from "mobx-react-lite";
import Text from "@components/Text";
import Balance from "@src/entities/Balance";
import Input from "@components/Input";
import TokenInfo from "./TokenInfo";

interface IProps {
  onClose: () => void;
  balances: Balance[];
  onSelect: (assetId: string) => void;
  visible: boolean;
  selectedTokenId?: string;
}

const TokenSelectModal: React.FC<IProps> = ({
  onClose,
  balances,
  onSelect,
  visible,
  selectedTokenId,
}) => {
  const [searchValue, setSearchValue] = useState<string>("");
  const handleSearch = (event: any) => {
    setSearchValue(event.target.value);
  };
  const ref = createRef<HTMLDivElement>();
  useOnClickOutside(ref, onClose);

  const handleTokenSelect = (assetId: string) => {
    onSelect(assetId);
    onClose();
  };
  const filteredTokens = balances.filter((v) => {
    if (!v || !v.symbol || !v.name) {
      return false;
    }
    return (
      v.symbol.toLowerCase().includes(searchValue.toLowerCase()) ||
      v.name.toLowerCase().includes(searchValue.toLowerCase())
    );
  });

  return (
    <Dialog
      visible={visible}
      style={{ maxWidth: 360 }}
      bodyStyle={{ minHeight: 440 }}
      onClose={() => {
        onClose();
      }}
      title="Select a token"
    >
      <Input
        value={searchValue}
        onChange={handleSearch}
        placeholder="Search by name or ticker…"
      />

      <SizedBox height={32} />
      <Scrollbar style={{ margin: -24 }}>
        <Column crossAxisSize="max" style={{ maxHeight: 352 }}>
          {filteredTokens && filteredTokens.length > 0 ? (
            filteredTokens.map((t) => {
              const disabled = selectedTokenId === t.assetId;
              return (
                <TokenInfo
                  hidden={disabled}
                  style={{ position: "relative" }}
                  withClickLogic
                  onClick={
                    !disabled ? () => handleTokenSelect(t.assetId) : () => null
                  }
                  key={t.assetId}
                  token={t}
                />
              );
            })
          ) : (
            <Text style={{ padding: "10px 24px" }}>No tokens found</Text>
          )}
          <SizedBox height={32} width={16} />
        </Column>
      </Scrollbar>
    </Dialog>
  );
};
export default observer(TokenSelectModal);
